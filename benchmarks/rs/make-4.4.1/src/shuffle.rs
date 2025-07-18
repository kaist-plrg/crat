use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type variable_set_list;
    pub type commands;
    static mut not_parallel: libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn fatal(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...) -> !;
    fn make_toui(_: *const libc::c_char, _: *mut *const libc::c_char) -> libc::c_uint;
    fn make_seed(_: libc::c_uint);
    fn make_rand() -> libc::c_uint;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uintmax_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const libc::c_char,
    pub lineno: libc::c_ulong,
    pub offset: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub mode: shuffle_mode,
    pub seed: libc::c_uint,
    pub shuffler: Option::<unsafe extern "C" fn(*mut *mut libc::c_void, size_t) -> ()>,
    pub strval: [libc::c_char; 23],
}
pub type shuffle_mode = libc::c_uint;
pub const sm_identity: shuffle_mode = 3;
pub const sm_reverse: shuffle_mode = 2;
pub const sm_random: shuffle_mode = 1;
pub const sm_none: shuffle_mode = 0;
static mut config: C2RustUnnamed = unsafe {
    {
        let mut init = C2RustUnnamed {
            mode: sm_none,
            seed: 0 as libc::c_int as libc::c_uint,
            shuffler: None,
            strval: *::std::mem::transmute::<
                &[u8; 23],
                &mut [libc::c_char; 23],
            >(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        };
        init
    }
};
pub unsafe extern "C" fn shuffle_get_mode() -> *const libc::c_char {
    return if config.strval[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
        0 as *mut libc::c_char
    } else {
        (config.strval).as_mut_ptr()
    };
}
pub unsafe extern "C" fn shuffle_set_mode(mut cmdarg: *const libc::c_char) {
    if strcasecmp(cmdarg, b"reverse\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        config.mode = sm_reverse;
        config
            .shuffler = Some(
            reverse_shuffle_array
                as unsafe extern "C" fn(*mut *mut libc::c_void, size_t) -> (),
        );
        strcpy(
            (config.strval).as_mut_ptr(),
            b"reverse\0" as *const u8 as *const libc::c_char,
        );
    } else if strcasecmp(cmdarg, b"identity\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        config.mode = sm_identity;
        config
            .shuffler = Some(
            identity_shuffle_array
                as unsafe extern "C" fn(*mut *mut libc::c_void, size_t) -> (),
        );
        strcpy(
            (config.strval).as_mut_ptr(),
            b"identity\0" as *const u8 as *const libc::c_char,
        );
    } else if strcasecmp(cmdarg, b"none\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        config.mode = sm_none;
        config.shuffler = None;
        config.strval[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    } else {
        if strcasecmp(cmdarg, b"random\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            config.seed = make_rand();
        } else {
            let mut err: *const libc::c_char = 0 as *const libc::c_char;
            config.seed = make_toui(cmdarg, &mut err);
            if !err.is_null() {
                fatal(
                    0 as *mut floc,
                    (strlen(err)).wrapping_add(strlen(cmdarg)),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid shuffle mode: %s: '%s'\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    err,
                    cmdarg,
                );
            }
        }
        config.mode = sm_random;
        config
            .shuffler = Some(
            random_shuffle_array
                as unsafe extern "C" fn(*mut *mut libc::c_void, size_t) -> (),
        );
        sprintf(
            (config.strval).as_mut_ptr(),
            b"%u\0" as *const u8 as *const libc::c_char,
            config.seed,
        );
    };
}
unsafe extern "C" fn random_shuffle_array(
    mut a: *mut *mut libc::c_void,
    mut len: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < len {
        let mut t: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut j: libc::c_uint = (make_rand() as libc::c_ulong).wrapping_rem(len)
            as libc::c_uint;
        if !(i == j as libc::c_ulong) {
            t = *a.offset(i as isize);
            let ref mut fresh0 = *a.offset(i as isize);
            *fresh0 = *a.offset(j as isize);
            let ref mut fresh1 = *a.offset(j as isize);
            *fresh1 = t;
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn reverse_shuffle_array(
    mut a: *mut *mut libc::c_void,
    mut len: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < len.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut t: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut j: size_t = len
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_sub(i);
        t = *a.offset(i as isize);
        let ref mut fresh2 = *a.offset(i as isize);
        *fresh2 = *a.offset(j as isize);
        let ref mut fresh3 = *a.offset(j as isize);
        *fresh3 = t;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn identity_shuffle_array(
    mut a: *mut *mut libc::c_void,
    mut len: size_t,
) {}
unsafe extern "C" fn shuffle_deps(mut deps: *mut dep) {
    let mut ndeps: size_t = 0 as libc::c_int as size_t;
    let mut dep: *mut dep = 0 as *mut dep;
    let mut da: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut dp: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    dep = deps;
    while !dep.is_null() {
        if (*dep).wait_here() != 0 {
            return;
        }
        ndeps = ndeps.wrapping_add(1);
        ndeps;
        dep = (*dep).next;
    }
    if ndeps == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    da = xmalloc(
        (::std::mem::size_of::<*mut dep>() as libc::c_ulong).wrapping_mul(ndeps),
    ) as *mut *mut libc::c_void;
    dep = deps;
    dp = da;
    while !dep.is_null() {
        *dp = dep as *mut libc::c_void;
        dep = (*dep).next;
        dp = dp.offset(1);
        dp;
    }
    (config.shuffler).unwrap()(da, ndeps);
    dep = deps;
    dp = da;
    while !dep.is_null() {
        (*dep).shuf = *dp as *mut dep;
        dep = (*dep).next;
        dp = dp.offset(1);
        dp;
    }
    free(da as *mut libc::c_void);
}
unsafe extern "C" fn shuffle_file_deps_recursive(mut f: *mut file) {
    let mut dep: *mut dep = 0 as *mut dep;
    if f.is_null() {
        return;
    }
    if (*f).was_shuffled() != 0 {
        return;
    }
    (*f).set_was_shuffled(1 as libc::c_int as libc::c_uint);
    shuffle_deps((*f).deps);
    dep = (*f).deps;
    while !dep.is_null() {
        shuffle_file_deps_recursive((*dep).file);
        dep = (*dep).next;
    }
}
pub unsafe extern "C" fn shuffle_deps_recursive(mut deps: *mut dep) {
    let mut dep: *mut dep = 0 as *mut dep;
    if config.mode as libc::c_uint == sm_none as libc::c_int as libc::c_uint {
        return;
    }
    if not_parallel != 0 {
        return;
    }
    if config.mode as libc::c_uint == sm_random as libc::c_int as libc::c_uint {
        make_seed(config.seed);
    }
    shuffle_deps(deps);
    dep = deps;
    while !dep.is_null() {
        shuffle_file_deps_recursive((*dep).file);
        dep = (*dep).next;
    }
}
