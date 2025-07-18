use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut stdout: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn theft_bloom_init(bit_size2: uint8_t) -> *mut theft_bloom;
    fn theft_bloom_mark(b: *mut theft_bloom, data: *mut uint8_t, data_size: size_t);
    fn theft_bloom_check(
        b: *mut theft_bloom,
        data: *mut uint8_t,
        data_size: size_t,
    ) -> bool;
    fn theft_bloom_free(b: *mut theft_bloom);
    fn theft_bloom_dump(b: *mut theft_bloom);
    fn theft_bloom_recommendation(trials: libc::c_int) -> uint8_t;
    fn theft_mt_init(seed: uint64_t) -> *mut theft_mt;
    fn theft_mt_free(mt: *mut theft_mt);
    fn theft_mt_reset(mt: *mut theft_mt, seed: uint64_t);
    fn theft_mt_random(mt: *mut theft_mt) -> uint64_t;
    fn theft_mt_random_double(mt: *mut theft_mt) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int16_t = __int16_t;
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
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type theft_seed = uint64_t;
pub type theft_hash = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_bloom {
    pub bit_count: uint8_t,
    pub size: size_t,
    pub bits: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_mt {
    pub mt: [uint64_t; 312],
    pub mti: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft {
    pub out: *mut FILE,
    pub seed: theft_seed,
    pub requested_bloom_bits: uint8_t,
    pub bloom: *mut theft_bloom,
    pub mt: *mut theft_mt,
}
pub type theft_alloc_cb = unsafe extern "C" fn(
    *mut theft,
    theft_seed,
    *mut libc::c_void,
) -> *mut libc::c_void;
pub type theft_free_cb = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
) -> ();
pub type theft_hash_cb = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
) -> theft_hash;
pub type theft_shrink_cb = unsafe extern "C" fn(
    *mut libc::c_void,
    uint32_t,
    *mut libc::c_void,
) -> *mut libc::c_void;
pub type theft_print_cb = unsafe extern "C" fn(
    *mut FILE,
    *mut libc::c_void,
    *mut libc::c_void,
) -> ();
pub type theft_trial_res = libc::c_uint;
pub const THEFT_TRIAL_ERROR: theft_trial_res = 4;
pub const THEFT_TRIAL_DUP: theft_trial_res = 3;
pub const THEFT_TRIAL_SKIP: theft_trial_res = 2;
pub const THEFT_TRIAL_FAIL: theft_trial_res = 1;
pub const THEFT_TRIAL_PASS: theft_trial_res = 0;
pub type theft_propfun = unsafe extern "C" fn() -> theft_trial_res;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_type_info {
    pub alloc: Option::<theft_alloc_cb>,
    pub free: Option::<theft_free_cb>,
    pub hash: Option::<theft_hash_cb>,
    pub shrink: Option::<theft_shrink_cb>,
    pub print: Option::<theft_print_cb>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_trial_info {
    pub name: *const libc::c_char,
    pub trial: libc::c_int,
    pub seed: theft_seed,
    pub status: theft_trial_res,
    pub arity: uint8_t,
    pub args: *mut *mut libc::c_void,
}
pub type theft_progress_callback_res = libc::c_uint;
pub const THEFT_PROGRESS_HALT: theft_progress_callback_res = 1;
pub const THEFT_PROGRESS_CONTINUE: theft_progress_callback_res = 0;
pub type theft_progress_cb = unsafe extern "C" fn(
    *mut theft_trial_info,
    *mut libc::c_void,
) -> theft_progress_callback_res;
pub type theft_run_res = libc::c_int;
pub const THEFT_RUN_ERROR_MISSING_CALLBACK: theft_run_res = -2;
pub const THEFT_RUN_ERROR_BAD_ARGS: theft_run_res = -1;
pub const THEFT_RUN_ERROR: theft_run_res = 2;
pub const THEFT_RUN_FAIL: theft_run_res = 1;
pub const THEFT_RUN_PASS: theft_run_res = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_trial_report {
    pub pass: size_t,
    pub fail: size_t,
    pub skip: size_t,
    pub dup: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_cfg {
    pub fun: Option::<theft_propfun>,
    pub type_info: [*mut theft_type_info; 10],
    pub name: *const libc::c_char,
    pub always_seed_count: libc::c_int,
    pub always_seeds: *mut theft_seed,
    pub trials: libc::c_int,
    pub progress_cb: Option::<theft_progress_cb>,
    pub env: *mut libc::c_void,
    pub report: *mut theft_trial_report,
    pub seed: theft_seed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_propfun_info {
    pub name: *const libc::c_char,
    pub fun: Option::<theft_propfun>,
    pub arity: uint8_t,
    pub type_info: [*mut theft_type_info; 10],
    pub always_seed_count: libc::c_int,
    pub always_seeds: *mut theft_seed,
}
pub const SHRINK_ERROR: shrink_res = 2;
pub const SHRINK_DEAD_END: shrink_res = 1;
pub const SHRINK_OK: shrink_res = 0;
pub type shrink_res = libc::c_uint;
pub const ALL_GEN_OK: all_gen_res_t = 0;
pub const ALL_GEN_ERROR: all_gen_res_t = 3;
pub const ALL_GEN_DUP: all_gen_res_t = 2;
pub const ALL_GEN_SKIP: all_gen_res_t = 1;
pub type all_gen_res_t = libc::c_uint;
pub unsafe extern "C" fn theft_init(mut bloom_bits: uint8_t) -> *mut theft {
    if bloom_bits as libc::c_int != 0 as libc::c_int
        && (bloom_bits as libc::c_int) < 13 as libc::c_int
        || bloom_bits as libc::c_int > 33 as libc::c_int
            && bloom_bits as libc::c_int != -(1 as libc::c_int) as uint8_t as libc::c_int
    {
        return 0 as *mut theft;
    }
    let mut t: *mut theft = malloc(::std::mem::size_of::<theft>() as libc::c_ulong)
        as *mut theft;
    if t.is_null() {
        return 0 as *mut theft;
    }
    memset(
        t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<theft>() as libc::c_ulong,
    );
    (*t).mt = theft_mt_init(0xa600d16b175eed as libc::c_long as uint64_t);
    if ((*t).mt).is_null() {
        free(t as *mut libc::c_void);
        return 0 as *mut theft;
    } else {
        (*t).out = stdout;
        (*t).requested_bloom_bits = bloom_bits;
        return t;
    };
}
pub unsafe extern "C" fn theft_set_seed(mut t: *mut theft, mut seed: uint64_t) {
    (*t).seed = seed;
    theft_mt_reset((*t).mt, seed);
}
pub unsafe extern "C" fn theft_random(mut t: *mut theft) -> theft_hash {
    let mut ns: theft_seed = theft_mt_random((*t).mt);
    return ns;
}
pub unsafe extern "C" fn theft_random_double(mut t: *mut theft) -> libc::c_double {
    return theft_mt_random_double((*t).mt);
}
pub unsafe extern "C" fn theft_set_output_stream(mut t: *mut theft, mut out: *mut FILE) {
    (*t).out = out;
}
unsafe extern "C" fn check_all_args(
    mut info: *mut theft_propfun_info,
    mut all_hashable: *mut bool,
) -> bool {
    let mut ah: bool = 1 as libc::c_int != 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*info).arity as libc::c_int {
        let mut ti: *mut theft_type_info = (*info).type_info[i as usize];
        if ((*ti).alloc).is_none() {
            return 0 as libc::c_int != 0;
        }
        if ((*ti).hash).is_none() {
            ah = 0 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    *all_hashable = ah;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn default_progress_cb(
    mut info: *mut theft_trial_info,
    mut env: *mut libc::c_void,
) -> theft_progress_callback_res {
    return THEFT_PROGRESS_CONTINUE;
}
unsafe extern "C" fn infer_arity(mut info: *mut theft_propfun_info) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        if ((*info).type_info[i as usize]).is_null() {
            (*info).arity = i as uint8_t;
            break;
        } else {
            i += 1;
            i;
        }
    }
}
pub unsafe extern "C" fn theft_run(
    mut t: *mut theft,
    mut cfg: *mut theft_cfg,
) -> theft_run_res {
    if t.is_null() || cfg.is_null() {
        return THEFT_RUN_ERROR_BAD_ARGS;
    }
    let mut info: theft_propfun_info = theft_propfun_info {
        name: 0 as *const libc::c_char,
        fun: None,
        arity: 0,
        type_info: [0 as *mut theft_type_info; 10],
        always_seed_count: 0,
        always_seeds: 0 as *mut theft_seed,
    };
    memset(
        &mut info as *mut theft_propfun_info as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<theft_propfun_info>() as libc::c_ulong,
    );
    info.name = (*cfg).name;
    info.fun = (*cfg).fun;
    memcpy(
        (info.type_info).as_mut_ptr() as *mut libc::c_void,
        ((*cfg).type_info).as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[*mut theft_type_info; 10]>() as libc::c_ulong,
    );
    info.always_seed_count = (*cfg).always_seed_count;
    info.always_seeds = (*cfg).always_seeds;
    if (*cfg).seed != 0 {
        theft_set_seed(t, (*cfg).seed);
    } else {
        theft_set_seed(t, 0xa600d16b175eed as libc::c_long as uint64_t);
    }
    if (*cfg).trials == 0 as libc::c_int {
        (*cfg).trials = 100 as libc::c_int;
    }
    return theft_run_internal(
        t,
        &mut info,
        (*cfg).trials,
        (*cfg).progress_cb,
        (*cfg).env,
        (*cfg).report,
    );
}
unsafe extern "C" fn theft_run_internal(
    mut t: *mut theft,
    mut info: *mut theft_propfun_info,
    mut trials: libc::c_int,
    mut cb: Option::<theft_progress_cb>,
    mut env: *mut libc::c_void,
    mut r: *mut theft_trial_report,
) -> theft_run_res {
    let mut fake_report: theft_trial_report = theft_trial_report {
        pass: 0,
        fail: 0,
        skip: 0,
        dup: 0,
    };
    if r.is_null() {
        r = &mut fake_report;
    }
    memset(
        r as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<theft_trial_report>() as libc::c_ulong,
    );
    infer_arity(info);
    if (*info).arity as libc::c_int == 0 as libc::c_int {
        return THEFT_RUN_ERROR_BAD_ARGS;
    }
    if t.is_null() || info.is_null() || ((*info).fun).is_none()
        || (*info).arity as libc::c_int == 0 as libc::c_int
    {
        return THEFT_RUN_ERROR_BAD_ARGS;
    }
    let mut all_hashable: bool = 0 as libc::c_int != 0;
    if !check_all_args(info, &mut all_hashable) {
        return THEFT_RUN_ERROR_MISSING_CALLBACK;
    }
    if cb.is_none() {
        cb = Some(
            default_progress_cb
                as unsafe extern "C" fn(
                    *mut theft_trial_info,
                    *mut libc::c_void,
                ) -> theft_progress_callback_res,
        );
    }
    if all_hashable {
        if (*t).requested_bloom_bits as libc::c_int == 0 as libc::c_int {
            (*t).requested_bloom_bits = theft_bloom_recommendation(trials);
        }
        if (*t).requested_bloom_bits as libc::c_int
            != -(1 as libc::c_int) as uint8_t as libc::c_int
        {
            (*t).bloom = theft_bloom_init((*t).requested_bloom_bits);
        }
    }
    let mut seed: theft_seed = (*t).seed;
    let mut initial_seed: theft_seed = (*t).seed;
    let mut always_seeds: libc::c_int = (*info).always_seed_count;
    if ((*info).always_seeds).is_null() {
        always_seeds = 0 as libc::c_int;
    }
    let mut args: [*mut libc::c_void; 10] = [0 as *mut libc::c_void; 10];
    let mut cres: theft_progress_callback_res = THEFT_PROGRESS_CONTINUE;
    let mut trial: libc::c_int = 0 as libc::c_int;
    while trial < trials {
        memset(
            args.as_mut_ptr() as *mut libc::c_void,
            0xff as libc::c_int,
            ::std::mem::size_of::<[*mut libc::c_void; 10]>() as libc::c_ulong,
        );
        if cres as libc::c_uint == THEFT_PROGRESS_HALT as libc::c_int as libc::c_uint {
            break;
        }
        if trial < always_seeds {
            seed = *((*info).always_seeds).offset(trial as isize);
        } else if always_seeds > 0 as libc::c_int && trial == always_seeds {
            seed = initial_seed;
        }
        let mut ti: theft_trial_info = {
            let mut init = theft_trial_info {
                name: (*info).name,
                trial: trial,
                seed: seed,
                status: THEFT_TRIAL_PASS,
                arity: (*info).arity,
                args: args.as_mut_ptr(),
            };
            init
        };
        theft_set_seed(t, seed);
        let mut gres: all_gen_res_t = gen_all_args(
            t,
            info,
            seed,
            args.as_mut_ptr(),
            env,
        );
        match gres as libc::c_uint {
            1 => {
                ti.status = THEFT_TRIAL_SKIP;
                (*r).skip = ((*r).skip).wrapping_add(1);
                (*r).skip;
                cres = cb.unwrap()(&mut ti, env);
            }
            2 => {
                ti.status = THEFT_TRIAL_DUP;
                (*r).dup = ((*r).dup).wrapping_add(1);
                (*r).dup;
                cres = cb.unwrap()(&mut ti, env);
            }
            0 => {
                if !run_trial(
                    t,
                    info,
                    args.as_mut_ptr(),
                    cb,
                    env,
                    r,
                    &mut ti,
                    &mut cres,
                ) {
                    return THEFT_RUN_ERROR;
                }
            }
            3 | _ => {
                ti.status = THEFT_TRIAL_ERROR;
                cres = cb.unwrap()(&mut ti, env);
                return THEFT_RUN_ERROR;
            }
        }
        free_args(info, args.as_mut_ptr(), env);
        theft_set_seed(t, seed);
        seed = theft_random(t);
        trial += 1;
        trial;
    }
    if (*r).fail > 0 as libc::c_int as libc::c_ulong {
        return THEFT_RUN_FAIL
    } else {
        return THEFT_RUN_PASS
    };
}
unsafe extern "C" fn run_trial(
    mut t: *mut theft,
    mut info: *mut theft_propfun_info,
    mut args: *mut *mut libc::c_void,
    mut cb: Option::<theft_progress_cb>,
    mut env: *mut libc::c_void,
    mut r: *mut theft_trial_report,
    mut ti: *mut theft_trial_info,
    mut cres: *mut theft_progress_callback_res,
) -> bool {
    if !((*t).bloom).is_null() {
        mark_called(t, info, args, env);
    }
    let mut tres: theft_trial_res = call_fun(info, args);
    (*ti).status = tres;
    match tres as libc::c_uint {
        0 => {
            (*r).pass = ((*r).pass).wrapping_add(1);
            (*r).pass;
            *cres = cb.unwrap()(ti, env);
        }
        1 => {
            if !attempt_to_shrink(t, info, args, env) {
                (*ti).status = THEFT_TRIAL_ERROR;
                *cres = cb.unwrap()(ti, env);
                return 0 as libc::c_int != 0;
            }
            (*r).fail = ((*r).fail).wrapping_add(1);
            (*r).fail;
            *cres = report_on_failure(t, info, ti, cb, env);
        }
        2 => {
            *cres = cb.unwrap()(ti, env);
            (*r).skip = ((*r).skip).wrapping_add(1);
            (*r).skip;
        }
        3 | 4 => {
            *cres = cb.unwrap()(ti, env);
            free_args(info, args, env);
            return 0 as libc::c_int != 0;
        }
        _ => {}
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn free_args(
    mut info: *mut theft_propfun_info,
    mut args: *mut *mut libc::c_void,
    mut env: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*info).arity as libc::c_int {
        let mut fcb: Option::<theft_free_cb> = (*(*info).type_info[i as usize]).free;
        if fcb.is_some()
            && *args.offset(i as isize) != -(1 as libc::c_int) as *mut libc::c_void
        {
            fcb.unwrap()(*args.offset(i as isize), env);
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn theft_free(mut t: *mut theft) {
    if !((*t).bloom).is_null() {
        theft_bloom_dump((*t).bloom);
        theft_bloom_free((*t).bloom);
        (*t).bloom = 0 as *mut theft_bloom;
    }
    theft_mt_free((*t).mt);
    free(t as *mut libc::c_void);
}
unsafe extern "C" fn call_fun(
    mut info: *mut theft_propfun_info,
    mut args: *mut *mut libc::c_void,
) -> theft_trial_res {
    let mut res: theft_trial_res = THEFT_TRIAL_ERROR;
    match (*info).arity as libc::c_int {
        1 => {
            res = ::std::mem::transmute::<
                _,
                fn(_) -> theft_trial_res,
            >(((*info).fun).unwrap())(*args.offset(0 as libc::c_int as isize));
        }
        2 => {
            res = ::std::mem::transmute::<
                _,
                fn(_, _) -> theft_trial_res,
            >(
                ((*info).fun).unwrap(),
            )(
                *args.offset(0 as libc::c_int as isize),
                *args.offset(1 as libc::c_int as isize),
            );
        }
        3 => {
            res = ::std::mem::transmute::<
                _,
                fn(_, _, _) -> theft_trial_res,
            >(
                ((*info).fun).unwrap(),
            )(
                *args.offset(0 as libc::c_int as isize),
                *args.offset(1 as libc::c_int as isize),
                *args.offset(2 as libc::c_int as isize),
            );
        }
        4 => {
            res = ::std::mem::transmute::<
                _,
                fn(_, _, _, _) -> theft_trial_res,
            >(
                ((*info).fun).unwrap(),
            )(
                *args.offset(0 as libc::c_int as isize),
                *args.offset(1 as libc::c_int as isize),
                *args.offset(2 as libc::c_int as isize),
                *args.offset(3 as libc::c_int as isize),
            );
        }
        5 => {
            res = ::std::mem::transmute::<
                _,
                fn(_, _, _, _, _) -> theft_trial_res,
            >(
                ((*info).fun).unwrap(),
            )(
                *args.offset(0 as libc::c_int as isize),
                *args.offset(1 as libc::c_int as isize),
                *args.offset(2 as libc::c_int as isize),
                *args.offset(3 as libc::c_int as isize),
                *args.offset(4 as libc::c_int as isize),
            );
        }
        6 => {
            res = ::std::mem::transmute::<
                _,
                fn(_, _, _, _, _, _) -> theft_trial_res,
            >(
                ((*info).fun).unwrap(),
            )(
                *args.offset(0 as libc::c_int as isize),
                *args.offset(1 as libc::c_int as isize),
                *args.offset(2 as libc::c_int as isize),
                *args.offset(3 as libc::c_int as isize),
                *args.offset(4 as libc::c_int as isize),
                *args.offset(5 as libc::c_int as isize),
            );
        }
        7 => {
            res = ::std::mem::transmute::<
                _,
                fn(_, _, _, _, _, _, _) -> theft_trial_res,
            >(
                ((*info).fun).unwrap(),
            )(
                *args.offset(0 as libc::c_int as isize),
                *args.offset(1 as libc::c_int as isize),
                *args.offset(2 as libc::c_int as isize),
                *args.offset(3 as libc::c_int as isize),
                *args.offset(4 as libc::c_int as isize),
                *args.offset(5 as libc::c_int as isize),
                *args.offset(6 as libc::c_int as isize),
            );
        }
        8 => {
            res = ::std::mem::transmute::<
                _,
                fn(_, _, _, _, _, _, _, _) -> theft_trial_res,
            >(
                ((*info).fun).unwrap(),
            )(
                *args.offset(0 as libc::c_int as isize),
                *args.offset(1 as libc::c_int as isize),
                *args.offset(2 as libc::c_int as isize),
                *args.offset(3 as libc::c_int as isize),
                *args.offset(4 as libc::c_int as isize),
                *args.offset(5 as libc::c_int as isize),
                *args.offset(6 as libc::c_int as isize),
                *args.offset(7 as libc::c_int as isize),
            );
        }
        9 => {
            res = ::std::mem::transmute::<
                _,
                fn(_, _, _, _, _, _, _, _, _) -> theft_trial_res,
            >(
                ((*info).fun).unwrap(),
            )(
                *args.offset(0 as libc::c_int as isize),
                *args.offset(1 as libc::c_int as isize),
                *args.offset(2 as libc::c_int as isize),
                *args.offset(3 as libc::c_int as isize),
                *args.offset(4 as libc::c_int as isize),
                *args.offset(5 as libc::c_int as isize),
                *args.offset(6 as libc::c_int as isize),
                *args.offset(7 as libc::c_int as isize),
                *args.offset(8 as libc::c_int as isize),
            );
        }
        10 => {
            res = ::std::mem::transmute::<
                _,
                fn(_, _, _, _, _, _, _, _, _, _) -> theft_trial_res,
            >(
                ((*info).fun).unwrap(),
            )(
                *args.offset(0 as libc::c_int as isize),
                *args.offset(1 as libc::c_int as isize),
                *args.offset(2 as libc::c_int as isize),
                *args.offset(3 as libc::c_int as isize),
                *args.offset(4 as libc::c_int as isize),
                *args.offset(5 as libc::c_int as isize),
                *args.offset(6 as libc::c_int as isize),
                *args.offset(7 as libc::c_int as isize),
                *args.offset(8 as libc::c_int as isize),
                *args.offset(9 as libc::c_int as isize),
            );
        }
        _ => return THEFT_TRIAL_ERROR,
    }
    return res;
}
unsafe extern "C" fn gen_all_args(
    mut t: *mut theft,
    mut info: *mut theft_propfun_info,
    mut seed: theft_seed,
    mut args: *mut *mut libc::c_void,
    mut env: *mut libc::c_void,
) -> all_gen_res_t {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*info).arity as libc::c_int {
        let mut ti: *mut theft_type_info = (*info).type_info[i as usize];
        let mut p: *mut libc::c_void = ((*ti).alloc).unwrap()(t, seed, env);
        if p == -(1 as libc::c_int) as *mut libc::c_void
            || p == -(2 as libc::c_int) as *mut libc::c_void
        {
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < i {
                ((*ti).free).unwrap()(*args.offset(j as isize), env);
                j += 1;
                j;
            }
            if p == -(1 as libc::c_int) as *mut libc::c_void {
                return ALL_GEN_SKIP
            } else {
                return ALL_GEN_ERROR
            }
        } else {
            let ref mut fresh0 = *args.offset(i as isize);
            *fresh0 = p;
        }
        seed = theft_random(t);
        i += 1;
        i;
    }
    if !((*t).bloom).is_null() && check_called(t, info, args, env) as libc::c_int != 0 {
        return ALL_GEN_DUP;
    }
    return ALL_GEN_OK;
}
unsafe extern "C" fn attempt_to_shrink(
    mut t: *mut theft,
    mut info: *mut theft_propfun_info,
    mut args: *mut *mut libc::c_void,
    mut env: *mut libc::c_void,
) -> bool {
    let mut progress: bool = 0 as libc::c_int != 0;
    loop {
        progress = 0 as libc::c_int != 0;
        let mut ai: libc::c_int = 0 as libc::c_int;
        while ai < (*info).arity as libc::c_int {
            let mut ti: *mut theft_type_info = (*info).type_info[ai as usize];
            if ((*ti).shrink).is_some() {
                let mut rres: shrink_res = attempt_to_shrink_arg(t, info, args, env, ai);
                match rres as libc::c_uint {
                    0 => {
                        progress = 1 as libc::c_int != 0;
                    }
                    1 => {}
                    2 | _ => return 0 as libc::c_int != 0,
                }
            }
            ai += 1;
            ai;
        }
        if !progress {
            break;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn attempt_to_shrink_arg(
    mut t: *mut theft,
    mut info: *mut theft_propfun_info,
    mut args: *mut *mut libc::c_void,
    mut env: *mut libc::c_void,
    mut ai: libc::c_int,
) -> shrink_res {
    let mut ti: *mut theft_type_info = (*info).type_info[ai as usize];
    let mut current_block_20: u64;
    let mut tactic: uint32_t = 0 as libc::c_int as uint32_t;
    while tactic < -(1 as libc::c_int) as uint32_t {
        let mut cur: *mut libc::c_void = *args.offset(ai as isize);
        let mut nv: *mut libc::c_void = ((*ti).shrink).unwrap()(cur, tactic, env);
        if nv == -(3 as libc::c_int) as *mut libc::c_void {
            return SHRINK_DEAD_END
        } else if nv == -(2 as libc::c_int) as *mut libc::c_void {
            return SHRINK_ERROR
        } else {
            if !(nv == -(1 as libc::c_int) as *mut libc::c_void) {
                let ref mut fresh1 = *args.offset(ai as isize);
                *fresh1 = nv;
                if !((*t).bloom).is_null() {
                    if check_called(t, info, args, env) {
                        if ((*ti).free).is_some() {
                            ((*ti).free).unwrap()(nv, env);
                        }
                        let ref mut fresh2 = *args.offset(ai as isize);
                        *fresh2 = cur;
                        current_block_20 = 11174649648027449784;
                    } else {
                        mark_called(t, info, args, env);
                        current_block_20 = 1054647088692577877;
                    }
                } else {
                    current_block_20 = 1054647088692577877;
                }
                match current_block_20 {
                    11174649648027449784 => {}
                    _ => {
                        let mut res: theft_trial_res = call_fun(info, args);
                        match res as libc::c_uint {
                            0 | 2 => {
                                let ref mut fresh3 = *args.offset(ai as isize);
                                *fresh3 = cur;
                                if ((*ti).free).is_some() {
                                    ((*ti).free).unwrap()(nv, env);
                                }
                            }
                            1 => {
                                if ((*ti).free).is_some() {
                                    ((*ti).free).unwrap()(cur, env);
                                }
                                return SHRINK_OK;
                            }
                            3 | 4 => return SHRINK_ERROR,
                            _ => {}
                        }
                    }
                }
            }
            tactic = tactic.wrapping_add(1);
            tactic;
        }
    }
    return SHRINK_DEAD_END;
}
unsafe extern "C" fn get_arg_hash_buffer(
    mut buffer: *mut theft_hash,
    mut info: *mut theft_propfun_info,
    mut args: *mut *mut libc::c_void,
    mut env: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*info).arity as libc::c_int {
        *buffer
            .offset(
                i as isize,
            ) = ((*(*info).type_info[i as usize]).hash)
            .unwrap()(*args.offset(i as isize), env);
        i += 1;
        i;
    }
}
unsafe extern "C" fn mark_called(
    mut t: *mut theft,
    mut info: *mut theft_propfun_info,
    mut args: *mut *mut libc::c_void,
    mut env: *mut libc::c_void,
) {
    let mut buffer: [theft_hash; 10] = [0; 10];
    get_arg_hash_buffer(buffer.as_mut_ptr(), info, args, env);
    theft_bloom_mark(
        (*t).bloom,
        buffer.as_mut_ptr() as *mut uint8_t,
        ((*info).arity as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<theft_hash>() as libc::c_ulong),
    );
}
unsafe extern "C" fn check_called(
    mut t: *mut theft,
    mut info: *mut theft_propfun_info,
    mut args: *mut *mut libc::c_void,
    mut env: *mut libc::c_void,
) -> bool {
    let mut buffer: [theft_hash; 10] = [0; 10];
    get_arg_hash_buffer(buffer.as_mut_ptr(), info, args, env);
    return theft_bloom_check(
        (*t).bloom,
        buffer.as_mut_ptr() as *mut uint8_t,
        ((*info).arity as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<theft_hash>() as libc::c_ulong),
    );
}
unsafe extern "C" fn report_on_failure(
    mut t: *mut theft,
    mut info: *mut theft_propfun_info,
    mut ti: *mut theft_trial_info,
    mut cb: Option::<theft_progress_cb>,
    mut env: *mut libc::c_void,
) -> theft_progress_callback_res {
    static mut cres: theft_progress_callback_res = THEFT_PROGRESS_CONTINUE;
    let mut arity: libc::c_int = (*info).arity as libc::c_int;
    fprintf(
        (*t).out,
        b"\n\n -- Counter-Example: %s\n\0" as *const u8 as *const libc::c_char,
        if !((*info).name).is_null() {
            (*info).name
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    fprintf(
        (*t).out,
        b"    Trial %u, Seed 0x%016llx\n\0" as *const u8 as *const libc::c_char,
        (*ti).trial,
        (*ti).seed,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < arity {
        let mut print: Option::<theft_print_cb> = (*(*info).type_info[i as usize]).print;
        if print.is_some() {
            fprintf(
                (*t).out,
                b"    Argument %d:\n\0" as *const u8 as *const libc::c_char,
                i,
            );
            print.unwrap()((*t).out, *((*ti).args).offset(i as isize), env);
            fprintf((*t).out, b"\n\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
        i;
    }
    cres = cb.unwrap()(ti, env);
    return cres;
}
