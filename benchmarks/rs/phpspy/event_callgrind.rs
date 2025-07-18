use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    static mut opt_path_output: *mut libc::c_char;
    fn log_error(fmt: *const libc::c_char, _: ...);
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type size_t = libc::c_ulong;
pub type pid_t = __pid_t;
pub type ptrdiff_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub struct UT_hash_bucket {
    pub hh_head: *mut UT_hash_handle,
    pub count: libc::c_uint,
    pub expand_mult: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_handle {
    pub tbl: *mut UT_hash_table,
    pub prev: *mut libc::c_void,
    pub next: *mut libc::c_void,
    pub hh_prev: *mut UT_hash_handle,
    pub hh_next: *mut UT_hash_handle,
    pub key: *mut libc::c_void,
    pub keylen: libc::c_uint,
    pub hashv: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_table {
    pub buckets: *mut UT_hash_bucket,
    pub num_buckets: libc::c_uint,
    pub log2_num_buckets: libc::c_uint,
    pub num_items: libc::c_uint,
    pub tail: *mut UT_hash_handle,
    pub hho: ptrdiff_t,
    pub ideal_chain_maxlen: libc::c_uint,
    pub nonideal_items: libc::c_uint,
    pub ineff_expands: libc::c_uint,
    pub noexpand: libc::c_uint,
    pub signature: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct varpeek_var_s {
    pub name: [libc::c_char; 256],
    pub hh: UT_hash_handle,
}
pub type varpeek_var_t = varpeek_var_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct varpeek_entry_s {
    pub filename_lineno: [libc::c_char; 256],
    pub varmap: *mut varpeek_var_t,
    pub hh: UT_hash_handle,
}
pub type varpeek_entry_t = varpeek_entry_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glopeek_entry_s {
    pub key: [libc::c_char; 256],
    pub gloname: [libc::c_char; 256],
    pub varname: [libc::c_char; 256],
    pub hh: UT_hash_handle,
}
pub type glopeek_entry_t = glopeek_entry_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trace_loc_s {
    pub func: [libc::c_char; 256],
    pub class: [libc::c_char; 256],
    pub file: [libc::c_char; 256],
    pub func_len: size_t,
    pub class_len: size_t,
    pub file_len: size_t,
    pub lineno: libc::c_int,
}
pub type trace_loc_t = trace_loc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trace_frame_s {
    pub loc: trace_loc_t,
    pub depth: libc::c_int,
}
pub type trace_frame_t = trace_frame_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trace_request_s {
    pub uri: [libc::c_char; 256],
    pub path: [libc::c_char; 256],
    pub qstring: [libc::c_char; 256],
    pub cookie: [libc::c_char; 256],
    pub ts: libc::c_double,
}
pub type trace_request_t = trace_request_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trace_mem_s {
    pub size: size_t,
    pub peak: size_t,
}
pub type trace_mem_t = trace_mem_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trace_varpeek_s {
    pub entry: *mut varpeek_entry_t,
    pub var: *mut varpeek_var_t,
    pub zval_str: *mut libc::c_char,
}
pub type trace_varpeek_t = trace_varpeek_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trace_glopeek_s {
    pub gentry: *mut glopeek_entry_t,
    pub zval_str: *mut libc::c_char,
}
pub type trace_glopeek_t = trace_glopeek_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trace_target_s {
    pub pid: pid_t,
    pub executor_globals_addr: uint64_t,
    pub sapi_globals_addr: uint64_t,
    pub alloc_globals_addr: uint64_t,
    pub basic_functions_module_addr: uint64_t,
}
pub type trace_target_t = trace_target_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trace_context_s {
    pub target: trace_target_t,
    pub event: C2RustUnnamed,
    pub event_udata: *mut libc::c_void,
    pub event_handler: Option::<
        unsafe extern "C" fn(*mut trace_context_s, libc::c_int) -> libc::c_int,
    >,
    pub buf: [libc::c_char; 256],
    pub buf_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub frame: trace_frame_t,
    pub request: trace_request_t,
    pub mem: trace_mem_t,
    pub varpeek: trace_varpeek_t,
    pub glopeek: trace_glopeek_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct callgrind_udata_t {
    pub self_0: [trace_loc_t; 128],
    pub prev: [trace_loc_t; 128],
    pub self_str: [[libc::c_char; 256]; 128],
    pub prev_str: [[libc::c_char; 256]; 128],
    pub self_len: libc::c_int,
    pub prev_len: libc::c_int,
    pub fout: *mut FILE,
    pub callers: *mut callgrind_caller_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct callgrind_caller_t {
    pub loc_str: [libc::c_char; 256],
    pub loc: trace_loc_t,
    pub exclusive: uint64_t,
    pub callees: *mut callgrind_callee_t,
    pub hh: UT_hash_handle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct callgrind_callee_t {
    pub loc_str: [libc::c_char; 256],
    pub loc: trace_loc_t,
    pub inclusive: uint64_t,
    pub count: uint64_t,
    pub hh: UT_hash_handle,
}
pub unsafe extern "C" fn event_handler_callgrind(
    mut context: *mut trace_context_s,
    mut event_type: libc::c_int,
) -> libc::c_int {
    let mut udata: *mut callgrind_udata_t = 0 as *mut callgrind_udata_t;
    udata = (*context).event_udata as *mut callgrind_udata_t;
    if udata.is_null() && event_type != 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    match event_type {
        0 => {
            udata = calloc(
                1 as libc::c_int as libc::c_ulong,
                ::std::mem::size_of::<callgrind_udata_t>() as libc::c_ulong,
            ) as *mut callgrind_udata_t;
            if callgrind_open(&mut (*udata).fout) != 0 as libc::c_int {
                free(udata as *mut libc::c_void);
                return 1 as libc::c_int;
            }
            (*context).event_udata = udata as *mut libc::c_void;
        }
        1 => {
            (*udata).self_len = 0 as libc::c_int;
        }
        2 => {
            callgrind_ingest_frame(udata, context);
        }
        7 => {
            callgrind_digest_stack(udata);
            memcpy(
                ((*udata).prev).as_mut_ptr() as *mut libc::c_void,
                ((*udata).self_0).as_mut_ptr() as *const libc::c_void,
                (::std::mem::size_of::<trace_loc_t>() as libc::c_ulong)
                    .wrapping_mul((*udata).self_len as libc::c_ulong),
            );
            memcpy(
                ((*udata).prev_str).as_mut_ptr() as *mut libc::c_void,
                ((*udata).self_str).as_mut_ptr() as *const libc::c_void,
                (256 as libc::c_int * (*udata).self_len) as libc::c_ulong,
            );
            (*udata).prev_len = (*udata).self_len;
            (*udata).self_len = 0 as libc::c_int;
        }
        9 => {
            callgrind_dump(udata);
            callgrind_free(udata);
            free(udata as *mut libc::c_void);
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn callgrind_ingest_frame(
    mut udata: *mut callgrind_udata_t,
    mut context: *mut trace_context_s,
) {
    if (*udata).self_len >= 128 as libc::c_int {
        log_error(
            b"callgrind_ingest_frame: Exceeded max stack depth (%d); truncating\n\0"
                as *const u8 as *const libc::c_char,
            128 as libc::c_int,
        );
        return;
    }
    memcpy(
        &mut *((*udata).self_0).as_mut_ptr().offset((*udata).self_len as isize)
            as *mut trace_loc_t as *mut libc::c_void,
        &mut (*context).event.frame.loc as *mut trace_loc_t as *const libc::c_void,
        ::std::mem::size_of::<trace_loc_t>() as libc::c_ulong,
    );
    callgrind_sprint_loc(
        ((*udata).self_str[(*udata).self_len as usize]).as_mut_ptr(),
        &mut (*context).event.frame.loc,
    );
    (*udata).self_len += 1 as libc::c_int;
}
unsafe extern "C" fn callgrind_sprint_loc(
    mut str: *mut libc::c_char,
    mut loc: *mut trace_loc_t,
) {
    let mut len: libc::c_int = 0;
    len = snprintf(
        str,
        256 as libc::c_int as libc::c_ulong,
        b"%.*s%s%.*s %.*s:%d\0" as *const u8 as *const libc::c_char,
        (*loc).class_len as libc::c_int,
        ((*loc).class).as_mut_ptr(),
        if (*loc).class_len > 0 as libc::c_int as libc::c_ulong {
            b"::\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        (*loc).func_len as libc::c_int,
        ((*loc).func).as_mut_ptr(),
        (*loc).file_len as libc::c_int,
        ((*loc).file).as_mut_ptr(),
        (*loc).lineno,
    );
    if len >= 256 as libc::c_int {
        log_error(
            b"callgrind_sprint_loc: Exceeded max loc len (%d); truncating\n\0"
                as *const u8 as *const libc::c_char,
            256 as libc::c_int,
        );
    }
}
unsafe extern "C" fn callgrind_digest_stack(mut udata: *mut callgrind_udata_t) {
    let mut caller: *mut callgrind_caller_t = 0 as *mut callgrind_caller_t;
    let mut prev_caller: *mut callgrind_caller_t = 0 as *mut callgrind_caller_t;
    let mut callee: *mut callgrind_callee_t = 0 as *mut callgrind_callee_t;
    let mut i: libc::c_int = 0;
    prev_caller = 0 as *mut callgrind_caller_t;
    i = (*udata).self_len - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut _uthash_hfstr_keylen: libc::c_uint = strlen(
            ((*udata).self_str[i as usize]).as_mut_ptr(),
        ) as libc::c_uint;
        let mut _hf_hashv: libc::c_uint = 0;
        let mut _hj_i: libc::c_uint = 0;
        let mut _hj_j: libc::c_uint = 0;
        let mut _hj_k: libc::c_uint = 0;
        let mut _hj_key: *const libc::c_uchar = ((*udata).self_str[i as usize])
            .as_mut_ptr() as *const libc::c_uchar;
        _hf_hashv = 0xfeedbeef as libc::c_uint;
        _hj_j = 0x9e3779b9 as libc::c_uint;
        _hj_i = _hj_j;
        _hj_k = _uthash_hfstr_keylen;
        while _hj_k >= 12 as libc::c_uint {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        ),
                );
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        ),
                );
            _hf_hashv = _hf_hashv
                .wrapping_add(
                    (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(11 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        ),
                );
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 13 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 8 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 13 as libc::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 12 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 16 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 5 as libc::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 3 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 10 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 15 as libc::c_int;
            _hj_key = _hj_key.offset(12 as libc::c_int as isize);
            _hj_k = _hj_k.wrapping_sub(12 as libc::c_uint);
        }
        _hf_hashv = _hf_hashv.wrapping_add(_uthash_hfstr_keylen);
        let mut current_block_52: u64;
        match _hj_k {
            11 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_52 = 12373616217255671355;
            }
            10 => {
                current_block_52 = 12373616217255671355;
            }
            9 => {
                current_block_52 = 324562901243291159;
            }
            8 => {
                current_block_52 = 10018103158551661172;
            }
            7 => {
                current_block_52 = 3001956548818886629;
            }
            6 => {
                current_block_52 = 13400492100268291696;
            }
            5 => {
                current_block_52 = 9594498520498683452;
            }
            4 => {
                current_block_52 = 7092856857215405885;
            }
            3 => {
                current_block_52 = 11104214808566124358;
            }
            2 => {
                current_block_52 = 8945840139371508841;
            }
            1 => {
                current_block_52 = 9536314573298548501;
            }
            _ => {
                current_block_52 = 12381812505308290051;
            }
        }
        match current_block_52 {
            12373616217255671355 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_52 = 324562901243291159;
            }
            _ => {}
        }
        match current_block_52 {
            324562901243291159 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_52 = 10018103158551661172;
            }
            _ => {}
        }
        match current_block_52 {
            10018103158551661172 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_52 = 3001956548818886629;
            }
            _ => {}
        }
        match current_block_52 {
            3001956548818886629 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_52 = 13400492100268291696;
            }
            _ => {}
        }
        match current_block_52 {
            13400492100268291696 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_52 = 9594498520498683452;
            }
            _ => {}
        }
        match current_block_52 {
            9594498520498683452 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_52 = 7092856857215405885;
            }
            _ => {}
        }
        match current_block_52 {
            7092856857215405885 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_52 = 11104214808566124358;
            }
            _ => {}
        }
        match current_block_52 {
            11104214808566124358 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_52 = 8945840139371508841;
            }
            _ => {}
        }
        match current_block_52 {
            8945840139371508841 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_52 = 9536314573298548501;
            }
            _ => {}
        }
        match current_block_52 {
            9536314573298548501 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        *_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint,
                    );
            }
            _ => {}
        }
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 13 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 13 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 12 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 5 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 3 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 15 as libc::c_int;
        caller = 0 as *mut callgrind_caller_t;
        if !((*udata).callers).is_null() {
            let mut _hf_bkt: libc::c_uint = 0;
            _hf_bkt = _hf_hashv
                & ((*(*(*udata).callers).hh.tbl).num_buckets)
                    .wrapping_sub(1 as libc::c_uint);
            if 1 as libc::c_int != 0 as libc::c_int {
                if !((*((*(*(*udata).callers).hh.tbl).buckets).offset(_hf_bkt as isize))
                    .hh_head)
                    .is_null()
                {
                    caller = ((*((*(*(*udata).callers).hh.tbl).buckets)
                        .offset(_hf_bkt as isize))
                        .hh_head as *mut libc::c_char)
                        .offset(-((*(*(*udata).callers).hh.tbl).hho as isize))
                        as *mut libc::c_void as *mut callgrind_caller_t;
                } else {
                    caller = 0 as *mut callgrind_caller_t;
                }
                while !caller.is_null() {
                    if (*caller).hh.hashv == _hf_hashv
                        && (*caller).hh.keylen == _uthash_hfstr_keylen
                    {
                        if memcmp(
                            (*caller).hh.key,
                            ((*udata).self_str[i as usize]).as_mut_ptr()
                                as *const libc::c_void,
                            _uthash_hfstr_keylen as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            break;
                        }
                    }
                    if !((*caller).hh.hh_next).is_null() {
                        caller = ((*caller).hh.hh_next as *mut libc::c_char)
                            .offset(-((*(*(*udata).callers).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut callgrind_caller_t;
                    } else {
                        caller = 0 as *mut callgrind_caller_t;
                    }
                }
            }
        }
        if caller.is_null() {
            caller = calloc(
                1 as libc::c_int as libc::c_ulong,
                ::std::mem::size_of::<callgrind_caller_t>() as libc::c_ulong,
            ) as *mut callgrind_caller_t;
            strcpy(
                ((*caller).loc_str).as_mut_ptr(),
                ((*udata).self_str[i as usize]).as_mut_ptr(),
            );
            memcpy(
                &mut (*caller).loc as *mut trace_loc_t as *mut libc::c_void,
                &mut *((*udata).self_0).as_mut_ptr().offset(i as isize)
                    as *mut trace_loc_t as *const libc::c_void,
                ::std::mem::size_of::<trace_loc_t>() as libc::c_ulong,
            );
            let mut _uthash_hastr_keylen: libc::c_uint = strlen(
                ((*caller).loc_str).as_mut_ptr(),
            ) as libc::c_uint;
            let mut _ha_hashv: libc::c_uint = 0;
            let mut _hj_i_0: libc::c_uint = 0;
            let mut _hj_j_0: libc::c_uint = 0;
            let mut _hj_k_0: libc::c_uint = 0;
            let mut _hj_key_0: *const libc::c_uchar = &mut *((*caller).loc_str)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_char
                as *const libc::c_uchar;
            _ha_hashv = 0xfeedbeef as libc::c_uint;
            _hj_j_0 = 0x9e3779b9 as libc::c_uint;
            _hj_i_0 = _hj_j_0;
            _hj_k_0 = _uthash_hastr_keylen;
            while _hj_k_0 >= 12 as libc::c_uint {
                _hj_i_0 = _hj_i_0
                    .wrapping_add(
                        (*_hj_key_0.offset(0 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*_hj_key_0.offset(1 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_0.offset(2 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_0.offset(3 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            ),
                    );
                _hj_j_0 = _hj_j_0
                    .wrapping_add(
                        (*_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*_hj_key_0.offset(5 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_0.offset(6 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_0.offset(7 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            ),
                    );
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key_0.offset(8 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*_hj_key_0.offset(9 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_0.offset(10 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_0.offset(11 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            ),
                    );
                _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                _hj_i_0 ^= _ha_hashv >> 13 as libc::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 8 as libc::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                _ha_hashv ^= _hj_j_0 >> 13 as libc::c_int;
                _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                _hj_i_0 ^= _ha_hashv >> 12 as libc::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 16 as libc::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                _ha_hashv ^= _hj_j_0 >> 5 as libc::c_int;
                _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                _hj_i_0 ^= _ha_hashv >> 3 as libc::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 10 as libc::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                _ha_hashv ^= _hj_j_0 >> 15 as libc::c_int;
                _hj_key_0 = _hj_key_0.offset(12 as libc::c_int as isize);
                _hj_k_0 = _hj_k_0.wrapping_sub(12 as libc::c_uint);
            }
            _ha_hashv = _ha_hashv.wrapping_add(_uthash_hastr_keylen);
            let mut current_block_171: u64;
            match _hj_k_0 {
                11 => {
                    _ha_hashv = _ha_hashv
                        .wrapping_add(
                            (*_hj_key_0.offset(10 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_171 = 18007573612332508338;
                }
                10 => {
                    current_block_171 = 18007573612332508338;
                }
                9 => {
                    current_block_171 = 9896855127789613117;
                }
                8 => {
                    current_block_171 = 11470911313929454839;
                }
                7 => {
                    current_block_171 = 2385085811710254279;
                }
                6 => {
                    current_block_171 = 4941045530350231705;
                }
                5 => {
                    current_block_171 = 774929567267066032;
                }
                4 => {
                    current_block_171 = 7993423040468902026;
                }
                3 => {
                    current_block_171 = 5289900845495047779;
                }
                2 => {
                    current_block_171 = 322004148427622879;
                }
                1 => {
                    current_block_171 = 18219220038645567589;
                }
                _ => {
                    current_block_171 = 7545150590528655645;
                }
            }
            match current_block_171 {
                18007573612332508338 => {
                    _ha_hashv = _ha_hashv
                        .wrapping_add(
                            (*_hj_key_0.offset(9 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_171 = 9896855127789613117;
                }
                _ => {}
            }
            match current_block_171 {
                9896855127789613117 => {
                    _ha_hashv = _ha_hashv
                        .wrapping_add(
                            (*_hj_key_0.offset(8 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_171 = 11470911313929454839;
                }
                _ => {}
            }
            match current_block_171 {
                11470911313929454839 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(
                            (*_hj_key_0.offset(7 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_171 = 2385085811710254279;
                }
                _ => {}
            }
            match current_block_171 {
                2385085811710254279 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(
                            (*_hj_key_0.offset(6 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_171 = 4941045530350231705;
                }
                _ => {}
            }
            match current_block_171 {
                4941045530350231705 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(
                            (*_hj_key_0.offset(5 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_171 = 774929567267066032;
                }
                _ => {}
            }
            match current_block_171 {
                774929567267066032 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(
                            *_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint,
                        );
                    current_block_171 = 7993423040468902026;
                }
                _ => {}
            }
            match current_block_171 {
                7993423040468902026 => {
                    _hj_i_0 = _hj_i_0
                        .wrapping_add(
                            (*_hj_key_0.offset(3 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_171 = 5289900845495047779;
                }
                _ => {}
            }
            match current_block_171 {
                5289900845495047779 => {
                    _hj_i_0 = _hj_i_0
                        .wrapping_add(
                            (*_hj_key_0.offset(2 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_171 = 322004148427622879;
                }
                _ => {}
            }
            match current_block_171 {
                322004148427622879 => {
                    _hj_i_0 = _hj_i_0
                        .wrapping_add(
                            (*_hj_key_0.offset(1 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_171 = 18219220038645567589;
                }
                _ => {}
            }
            match current_block_171 {
                18219220038645567589 => {
                    _hj_i_0 = _hj_i_0
                        .wrapping_add(
                            *_hj_key_0.offset(0 as libc::c_int as isize) as libc::c_uint,
                        );
                }
                _ => {}
            }
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 13 as libc::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 8 as libc::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 13 as libc::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 12 as libc::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 16 as libc::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 5 as libc::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 3 as libc::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 10 as libc::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 15 as libc::c_int;
            (*caller).hh.hashv = _ha_hashv;
            (*caller)
                .hh
                .key = &mut *((*caller).loc_str)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_char
                as *mut libc::c_void;
            (*caller).hh.keylen = _uthash_hastr_keylen;
            if ((*udata).callers).is_null() {
                (*caller).hh.next = 0 as *mut libc::c_void;
                (*caller).hh.prev = 0 as *mut libc::c_void;
                (*caller)
                    .hh
                    .tbl = malloc(
                    ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                ) as *mut UT_hash_table;
                if ((*caller).hh.tbl).is_null() {
                    exit(-(1 as libc::c_int));
                } else {
                    memset(
                        (*caller).hh.tbl as *mut libc::c_void,
                        '\0' as i32,
                        ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                    );
                    (*(*caller).hh.tbl).tail = &mut (*caller).hh;
                    (*(*caller).hh.tbl).num_buckets = 32 as libc::c_uint;
                    (*(*caller).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
                    (*(*caller).hh.tbl)
                        .hho = (&mut (*caller).hh as *mut UT_hash_handle
                        as *mut libc::c_char)
                        .offset_from(caller as *mut libc::c_char) as libc::c_long;
                    (*(*caller).hh.tbl)
                        .buckets = malloc(
                        (32 as libc::c_uint as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                            ),
                    ) as *mut UT_hash_bucket;
                    (*(*caller).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
                    if ((*(*caller).hh.tbl).buckets).is_null() {
                        exit(-(1 as libc::c_int));
                    } else {
                        memset(
                            (*(*caller).hh.tbl).buckets as *mut libc::c_void,
                            '\0' as i32,
                            (32 as libc::c_uint as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                                ),
                        );
                    }
                }
                (*udata).callers = caller;
            } else {
                (*caller).hh.tbl = (*(*udata).callers).hh.tbl;
                (*caller).hh.next = 0 as *mut libc::c_void;
                (*caller)
                    .hh
                    .prev = ((*(*(*udata).callers).hh.tbl).tail as *mut libc::c_char)
                    .offset(-((*(*(*udata).callers).hh.tbl).hho as isize))
                    as *mut libc::c_void;
                (*(*(*(*udata).callers).hh.tbl).tail).next = caller as *mut libc::c_void;
                (*(*(*udata).callers).hh.tbl).tail = &mut (*caller).hh;
            }
            let mut _ha_bkt: libc::c_uint = 0;
            (*(*(*udata).callers).hh.tbl)
                .num_items = ((*(*(*udata).callers).hh.tbl).num_items).wrapping_add(1);
            (*(*(*udata).callers).hh.tbl).num_items;
            _ha_bkt = _ha_hashv
                & ((*(*(*udata).callers).hh.tbl).num_buckets)
                    .wrapping_sub(1 as libc::c_uint);
            let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*(*udata).callers).hh.tbl)
                .buckets)
                .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
            (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
            (*_ha_head).count;
            (*caller).hh.hh_next = (*_ha_head).hh_head;
            (*caller).hh.hh_prev = 0 as *mut UT_hash_handle;
            if !((*_ha_head).hh_head).is_null() {
                (*(*_ha_head).hh_head).hh_prev = &mut (*caller).hh;
            }
            (*_ha_head).hh_head = &mut (*caller).hh;
            if (*_ha_head).count
                >= ((*_ha_head).expand_mult)
                    .wrapping_add(1 as libc::c_uint)
                    .wrapping_mul(10 as libc::c_uint)
                && (*(*caller).hh.tbl).noexpand == 0
            {
                let mut _he_bkt: libc::c_uint = 0;
                let mut _he_bkt_i: libc::c_uint = 0;
                let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
                let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
                let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
                let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
                _he_new_buckets = malloc(
                    (2 as libc::c_ulong)
                        .wrapping_mul((*(*caller).hh.tbl).num_buckets as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                ) as *mut UT_hash_bucket;
                if _he_new_buckets.is_null() {
                    exit(-(1 as libc::c_int));
                } else {
                    memset(
                        _he_new_buckets as *mut libc::c_void,
                        '\0' as i32,
                        (2 as libc::c_ulong)
                            .wrapping_mul(
                                (*(*caller).hh.tbl).num_buckets as libc::c_ulong,
                            )
                            .wrapping_mul(
                                ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                            ),
                    );
                    (*(*caller).hh.tbl)
                        .ideal_chain_maxlen = ((*(*caller).hh.tbl).num_items
                        >> ((*(*caller).hh.tbl).log2_num_buckets)
                            .wrapping_add(1 as libc::c_uint))
                        .wrapping_add(
                            (if (*(*caller).hh.tbl).num_items
                                & ((*(*caller).hh.tbl).num_buckets)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                            {
                                1 as libc::c_uint
                            } else {
                                0 as libc::c_uint
                            }),
                        );
                    (*(*caller).hh.tbl)
                        .nonideal_items = 0 as libc::c_int as libc::c_uint;
                    _he_bkt_i = 0 as libc::c_int as libc::c_uint;
                    while _he_bkt_i < (*(*caller).hh.tbl).num_buckets {
                        _he_thh = (*((*(*caller).hh.tbl).buckets)
                            .offset(_he_bkt_i as isize))
                            .hh_head;
                        while !_he_thh.is_null() {
                            _he_hh_nxt = (*_he_thh).hh_next;
                            _he_bkt = (*_he_thh).hashv
                                & ((*(*caller).hh.tbl).num_buckets)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_sub(1 as libc::c_uint);
                            _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                                as *mut UT_hash_bucket;
                            (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                            if (*_he_newbkt).count
                                > (*(*caller).hh.tbl).ideal_chain_maxlen
                            {
                                (*(*caller).hh.tbl)
                                    .nonideal_items = ((*(*caller).hh.tbl).nonideal_items)
                                    .wrapping_add(1);
                                (*(*caller).hh.tbl).nonideal_items;
                                (*_he_newbkt)
                                    .expand_mult = ((*_he_newbkt).count)
                                    .wrapping_div((*(*caller).hh.tbl).ideal_chain_maxlen);
                            }
                            (*_he_thh).hh_prev = 0 as *mut UT_hash_handle;
                            (*_he_thh).hh_next = (*_he_newbkt).hh_head;
                            if !((*_he_newbkt).hh_head).is_null() {
                                (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                            }
                            (*_he_newbkt).hh_head = _he_thh;
                            _he_thh = _he_hh_nxt;
                        }
                        _he_bkt_i = _he_bkt_i.wrapping_add(1);
                        _he_bkt_i;
                    }
                    free((*(*caller).hh.tbl).buckets as *mut libc::c_void);
                    (*(*caller).hh.tbl)
                        .num_buckets = ((*(*caller).hh.tbl).num_buckets)
                        .wrapping_mul(2 as libc::c_uint);
                    (*(*caller).hh.tbl)
                        .log2_num_buckets = ((*(*caller).hh.tbl).log2_num_buckets)
                        .wrapping_add(1);
                    (*(*caller).hh.tbl).log2_num_buckets;
                    (*(*caller).hh.tbl).buckets = _he_new_buckets;
                    (*(*caller).hh.tbl)
                        .ineff_expands = if (*(*caller).hh.tbl).nonideal_items
                        > (*(*caller).hh.tbl).num_items >> 1 as libc::c_int
                    {
                        ((*(*caller).hh.tbl).ineff_expands)
                            .wrapping_add(1 as libc::c_uint)
                    } else {
                        0 as libc::c_uint
                    };
                    if (*(*caller).hh.tbl).ineff_expands > 1 as libc::c_uint {
                        (*(*caller).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
                    }
                }
            }
        }
        if i == 0 as libc::c_int {
            (*caller)
                .exclusive = ((*caller).exclusive as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as uint64_t as uint64_t;
        }
        if !prev_caller.is_null() {
            let mut _uthash_hfstr_keylen_0: libc::c_uint = strlen(
                ((*udata).self_str[i as usize]).as_mut_ptr(),
            ) as libc::c_uint;
            let mut _hf_hashv_0: libc::c_uint = 0;
            let mut _hj_i_1: libc::c_uint = 0;
            let mut _hj_j_1: libc::c_uint = 0;
            let mut _hj_k_1: libc::c_uint = 0;
            let mut _hj_key_1: *const libc::c_uchar = ((*udata).self_str[i as usize])
                .as_mut_ptr() as *const libc::c_uchar;
            _hf_hashv_0 = 0xfeedbeef as libc::c_uint;
            _hj_j_1 = 0x9e3779b9 as libc::c_uint;
            _hj_i_1 = _hj_j_1;
            _hj_k_1 = _uthash_hfstr_keylen_0;
            while _hj_k_1 >= 12 as libc::c_uint {
                _hj_i_1 = _hj_i_1
                    .wrapping_add(
                        (*_hj_key_1.offset(0 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*_hj_key_1.offset(1 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_1.offset(2 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_1.offset(3 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            ),
                    );
                _hj_j_1 = _hj_j_1
                    .wrapping_add(
                        (*_hj_key_1.offset(4 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*_hj_key_1.offset(5 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_1.offset(6 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_1.offset(7 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            ),
                    );
                _hf_hashv_0 = _hf_hashv_0
                    .wrapping_add(
                        (*_hj_key_1.offset(8 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*_hj_key_1.offset(9 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_1.offset(10 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_1.offset(11 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            ),
                    );
                _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
                _hj_i_1 ^= _hf_hashv_0 >> 13 as libc::c_int;
                _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
                _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                _hj_j_1 ^= _hj_i_1 << 8 as libc::c_int;
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
                _hf_hashv_0 ^= _hj_j_1 >> 13 as libc::c_int;
                _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
                _hj_i_1 ^= _hf_hashv_0 >> 12 as libc::c_int;
                _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
                _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                _hj_j_1 ^= _hj_i_1 << 16 as libc::c_int;
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
                _hf_hashv_0 ^= _hj_j_1 >> 5 as libc::c_int;
                _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
                _hj_i_1 ^= _hf_hashv_0 >> 3 as libc::c_int;
                _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
                _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                _hj_j_1 ^= _hj_i_1 << 10 as libc::c_int;
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
                _hf_hashv_0 ^= _hj_j_1 >> 15 as libc::c_int;
                _hj_key_1 = _hj_key_1.offset(12 as libc::c_int as isize);
                _hj_k_1 = _hj_k_1.wrapping_sub(12 as libc::c_uint);
            }
            _hf_hashv_0 = _hf_hashv_0.wrapping_add(_uthash_hfstr_keylen_0);
            let mut current_block_366: u64;
            match _hj_k_1 {
                11 => {
                    _hf_hashv_0 = _hf_hashv_0
                        .wrapping_add(
                            (*_hj_key_1.offset(10 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_366 = 2260452682697560094;
                }
                10 => {
                    current_block_366 = 2260452682697560094;
                }
                9 => {
                    current_block_366 = 5843925138985246205;
                }
                8 => {
                    current_block_366 = 4866546000086821080;
                }
                7 => {
                    current_block_366 = 4039540690446938694;
                }
                6 => {
                    current_block_366 = 17806910502758698150;
                }
                5 => {
                    current_block_366 = 5178503495047205854;
                }
                4 => {
                    current_block_366 = 10392946923047303939;
                }
                3 => {
                    current_block_366 = 3383103829304262926;
                }
                2 => {
                    current_block_366 = 15275504483753031639;
                }
                1 => {
                    current_block_366 = 14514390918547092561;
                }
                _ => {
                    current_block_366 = 8460857742761360582;
                }
            }
            match current_block_366 {
                2260452682697560094 => {
                    _hf_hashv_0 = _hf_hashv_0
                        .wrapping_add(
                            (*_hj_key_1.offset(9 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_366 = 5843925138985246205;
                }
                _ => {}
            }
            match current_block_366 {
                5843925138985246205 => {
                    _hf_hashv_0 = _hf_hashv_0
                        .wrapping_add(
                            (*_hj_key_1.offset(8 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_366 = 4866546000086821080;
                }
                _ => {}
            }
            match current_block_366 {
                4866546000086821080 => {
                    _hj_j_1 = _hj_j_1
                        .wrapping_add(
                            (*_hj_key_1.offset(7 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_366 = 4039540690446938694;
                }
                _ => {}
            }
            match current_block_366 {
                4039540690446938694 => {
                    _hj_j_1 = _hj_j_1
                        .wrapping_add(
                            (*_hj_key_1.offset(6 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_366 = 17806910502758698150;
                }
                _ => {}
            }
            match current_block_366 {
                17806910502758698150 => {
                    _hj_j_1 = _hj_j_1
                        .wrapping_add(
                            (*_hj_key_1.offset(5 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_366 = 5178503495047205854;
                }
                _ => {}
            }
            match current_block_366 {
                5178503495047205854 => {
                    _hj_j_1 = _hj_j_1
                        .wrapping_add(
                            *_hj_key_1.offset(4 as libc::c_int as isize) as libc::c_uint,
                        );
                    current_block_366 = 10392946923047303939;
                }
                _ => {}
            }
            match current_block_366 {
                10392946923047303939 => {
                    _hj_i_1 = _hj_i_1
                        .wrapping_add(
                            (*_hj_key_1.offset(3 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_366 = 3383103829304262926;
                }
                _ => {}
            }
            match current_block_366 {
                3383103829304262926 => {
                    _hj_i_1 = _hj_i_1
                        .wrapping_add(
                            (*_hj_key_1.offset(2 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_366 = 15275504483753031639;
                }
                _ => {}
            }
            match current_block_366 {
                15275504483753031639 => {
                    _hj_i_1 = _hj_i_1
                        .wrapping_add(
                            (*_hj_key_1.offset(1 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_366 = 14514390918547092561;
                }
                _ => {}
            }
            match current_block_366 {
                14514390918547092561 => {
                    _hj_i_1 = _hj_i_1
                        .wrapping_add(
                            *_hj_key_1.offset(0 as libc::c_int as isize) as libc::c_uint,
                        );
                }
                _ => {}
            }
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
            _hj_i_1 ^= _hf_hashv_0 >> 13 as libc::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 8 as libc::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
            _hf_hashv_0 ^= _hj_j_1 >> 13 as libc::c_int;
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
            _hj_i_1 ^= _hf_hashv_0 >> 12 as libc::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 16 as libc::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
            _hf_hashv_0 ^= _hj_j_1 >> 5 as libc::c_int;
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
            _hj_i_1 ^= _hf_hashv_0 >> 3 as libc::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 10 as libc::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
            _hf_hashv_0 ^= _hj_j_1 >> 15 as libc::c_int;
            callee = 0 as *mut callgrind_callee_t;
            if !((*prev_caller).callees).is_null() {
                let mut _hf_bkt_0: libc::c_uint = 0;
                _hf_bkt_0 = _hf_hashv_0
                    & ((*(*(*prev_caller).callees).hh.tbl).num_buckets)
                        .wrapping_sub(1 as libc::c_uint);
                if 1 as libc::c_int != 0 as libc::c_int {
                    if !((*((*(*(*prev_caller).callees).hh.tbl).buckets)
                        .offset(_hf_bkt_0 as isize))
                        .hh_head)
                        .is_null()
                    {
                        callee = ((*((*(*(*prev_caller).callees).hh.tbl).buckets)
                            .offset(_hf_bkt_0 as isize))
                            .hh_head as *mut libc::c_char)
                            .offset(-((*(*(*prev_caller).callees).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut callgrind_callee_t;
                    } else {
                        callee = 0 as *mut callgrind_callee_t;
                    }
                    while !callee.is_null() {
                        if (*callee).hh.hashv == _hf_hashv_0
                            && (*callee).hh.keylen == _uthash_hfstr_keylen_0
                        {
                            if memcmp(
                                (*callee).hh.key,
                                ((*udata).self_str[i as usize]).as_mut_ptr()
                                    as *const libc::c_void,
                                _uthash_hfstr_keylen_0 as libc::c_ulong,
                            ) == 0 as libc::c_int
                            {
                                break;
                            }
                        }
                        if !((*callee).hh.hh_next).is_null() {
                            callee = ((*callee).hh.hh_next as *mut libc::c_char)
                                .offset(-((*(*(*prev_caller).callees).hh.tbl).hho as isize))
                                as *mut libc::c_void as *mut callgrind_callee_t;
                        } else {
                            callee = 0 as *mut callgrind_callee_t;
                        }
                    }
                }
            }
            if callee.is_null() {
                callee = calloc(
                    1 as libc::c_int as libc::c_ulong,
                    ::std::mem::size_of::<callgrind_callee_t>() as libc::c_ulong,
                ) as *mut callgrind_callee_t;
                strcpy(
                    ((*callee).loc_str).as_mut_ptr(),
                    ((*udata).self_str[i as usize]).as_mut_ptr(),
                );
                memcpy(
                    &mut (*callee).loc as *mut trace_loc_t as *mut libc::c_void,
                    &mut *((*udata).self_0).as_mut_ptr().offset(i as isize)
                        as *mut trace_loc_t as *const libc::c_void,
                    ::std::mem::size_of::<trace_loc_t>() as libc::c_ulong,
                );
                let mut _uthash_hastr_keylen_0: libc::c_uint = strlen(
                    ((*callee).loc_str).as_mut_ptr(),
                ) as libc::c_uint;
                let mut _ha_hashv_0: libc::c_uint = 0;
                let mut _hj_i_2: libc::c_uint = 0;
                let mut _hj_j_2: libc::c_uint = 0;
                let mut _hj_k_2: libc::c_uint = 0;
                let mut _hj_key_2: *const libc::c_uchar = &mut *((*callee).loc_str)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut libc::c_char
                    as *const libc::c_uchar;
                _ha_hashv_0 = 0xfeedbeef as libc::c_uint;
                _hj_j_2 = 0x9e3779b9 as libc::c_uint;
                _hj_i_2 = _hj_j_2;
                _hj_k_2 = _uthash_hastr_keylen_0;
                while _hj_k_2 >= 12 as libc::c_uint {
                    _hj_i_2 = _hj_i_2
                        .wrapping_add(
                            (*_hj_key_2.offset(0 as libc::c_int as isize)
                                as libc::c_uint)
                                .wrapping_add(
                                    (*_hj_key_2.offset(1 as libc::c_int as isize)
                                        as libc::c_uint) << 8 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_2.offset(2 as libc::c_int as isize)
                                        as libc::c_uint) << 16 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_2.offset(3 as libc::c_int as isize)
                                        as libc::c_uint) << 24 as libc::c_int,
                                ),
                        );
                    _hj_j_2 = _hj_j_2
                        .wrapping_add(
                            (*_hj_key_2.offset(4 as libc::c_int as isize)
                                as libc::c_uint)
                                .wrapping_add(
                                    (*_hj_key_2.offset(5 as libc::c_int as isize)
                                        as libc::c_uint) << 8 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_2.offset(6 as libc::c_int as isize)
                                        as libc::c_uint) << 16 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_2.offset(7 as libc::c_int as isize)
                                        as libc::c_uint) << 24 as libc::c_int,
                                ),
                        );
                    _ha_hashv_0 = _ha_hashv_0
                        .wrapping_add(
                            (*_hj_key_2.offset(8 as libc::c_int as isize)
                                as libc::c_uint)
                                .wrapping_add(
                                    (*_hj_key_2.offset(9 as libc::c_int as isize)
                                        as libc::c_uint) << 8 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_2.offset(10 as libc::c_int as isize)
                                        as libc::c_uint) << 16 as libc::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_2.offset(11 as libc::c_int as isize)
                                        as libc::c_uint) << 24 as libc::c_int,
                                ),
                        );
                    _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                    _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv_0);
                    _hj_i_2 ^= _ha_hashv_0 >> 13 as libc::c_int;
                    _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv_0);
                    _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                    _hj_j_2 ^= _hj_i_2 << 8 as libc::c_int;
                    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_2);
                    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_2);
                    _ha_hashv_0 ^= _hj_j_2 >> 13 as libc::c_int;
                    _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                    _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv_0);
                    _hj_i_2 ^= _ha_hashv_0 >> 12 as libc::c_int;
                    _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv_0);
                    _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                    _hj_j_2 ^= _hj_i_2 << 16 as libc::c_int;
                    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_2);
                    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_2);
                    _ha_hashv_0 ^= _hj_j_2 >> 5 as libc::c_int;
                    _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                    _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv_0);
                    _hj_i_2 ^= _ha_hashv_0 >> 3 as libc::c_int;
                    _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv_0);
                    _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                    _hj_j_2 ^= _hj_i_2 << 10 as libc::c_int;
                    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_2);
                    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_2);
                    _ha_hashv_0 ^= _hj_j_2 >> 15 as libc::c_int;
                    _hj_key_2 = _hj_key_2.offset(12 as libc::c_int as isize);
                    _hj_k_2 = _hj_k_2.wrapping_sub(12 as libc::c_uint);
                }
                _ha_hashv_0 = _ha_hashv_0.wrapping_add(_uthash_hastr_keylen_0);
                let mut current_block_485: u64;
                match _hj_k_2 {
                    11 => {
                        _ha_hashv_0 = _ha_hashv_0
                            .wrapping_add(
                                (*_hj_key_2.offset(10 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            );
                        current_block_485 = 17915696866079319890;
                    }
                    10 => {
                        current_block_485 = 17915696866079319890;
                    }
                    9 => {
                        current_block_485 = 7580955447532103554;
                    }
                    8 => {
                        current_block_485 = 9830530941200640175;
                    }
                    7 => {
                        current_block_485 = 1791461474802128541;
                    }
                    6 => {
                        current_block_485 = 15066782448046541784;
                    }
                    5 => {
                        current_block_485 = 15470920164401962094;
                    }
                    4 => {
                        current_block_485 = 4098275323032176464;
                    }
                    3 => {
                        current_block_485 = 1014462587972260425;
                    }
                    2 => {
                        current_block_485 = 15540172209514651165;
                    }
                    1 => {
                        current_block_485 = 11993601432291724909;
                    }
                    _ => {
                        current_block_485 = 14933967110489461578;
                    }
                }
                match current_block_485 {
                    17915696866079319890 => {
                        _ha_hashv_0 = _ha_hashv_0
                            .wrapping_add(
                                (*_hj_key_2.offset(9 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            );
                        current_block_485 = 7580955447532103554;
                    }
                    _ => {}
                }
                match current_block_485 {
                    7580955447532103554 => {
                        _ha_hashv_0 = _ha_hashv_0
                            .wrapping_add(
                                (*_hj_key_2.offset(8 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            );
                        current_block_485 = 9830530941200640175;
                    }
                    _ => {}
                }
                match current_block_485 {
                    9830530941200640175 => {
                        _hj_j_2 = _hj_j_2
                            .wrapping_add(
                                (*_hj_key_2.offset(7 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            );
                        current_block_485 = 1791461474802128541;
                    }
                    _ => {}
                }
                match current_block_485 {
                    1791461474802128541 => {
                        _hj_j_2 = _hj_j_2
                            .wrapping_add(
                                (*_hj_key_2.offset(6 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            );
                        current_block_485 = 15066782448046541784;
                    }
                    _ => {}
                }
                match current_block_485 {
                    15066782448046541784 => {
                        _hj_j_2 = _hj_j_2
                            .wrapping_add(
                                (*_hj_key_2.offset(5 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            );
                        current_block_485 = 15470920164401962094;
                    }
                    _ => {}
                }
                match current_block_485 {
                    15470920164401962094 => {
                        _hj_j_2 = _hj_j_2
                            .wrapping_add(
                                *_hj_key_2.offset(4 as libc::c_int as isize) as libc::c_uint,
                            );
                        current_block_485 = 4098275323032176464;
                    }
                    _ => {}
                }
                match current_block_485 {
                    4098275323032176464 => {
                        _hj_i_2 = _hj_i_2
                            .wrapping_add(
                                (*_hj_key_2.offset(3 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            );
                        current_block_485 = 1014462587972260425;
                    }
                    _ => {}
                }
                match current_block_485 {
                    1014462587972260425 => {
                        _hj_i_2 = _hj_i_2
                            .wrapping_add(
                                (*_hj_key_2.offset(2 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            );
                        current_block_485 = 15540172209514651165;
                    }
                    _ => {}
                }
                match current_block_485 {
                    15540172209514651165 => {
                        _hj_i_2 = _hj_i_2
                            .wrapping_add(
                                (*_hj_key_2.offset(1 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            );
                        current_block_485 = 11993601432291724909;
                    }
                    _ => {}
                }
                match current_block_485 {
                    11993601432291724909 => {
                        _hj_i_2 = _hj_i_2
                            .wrapping_add(
                                *_hj_key_2.offset(0 as libc::c_int as isize) as libc::c_uint,
                            );
                    }
                    _ => {}
                }
                _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv_0);
                _hj_i_2 ^= _ha_hashv_0 >> 13 as libc::c_int;
                _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv_0);
                _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                _hj_j_2 ^= _hj_i_2 << 8 as libc::c_int;
                _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_2);
                _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_2);
                _ha_hashv_0 ^= _hj_j_2 >> 13 as libc::c_int;
                _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv_0);
                _hj_i_2 ^= _ha_hashv_0 >> 12 as libc::c_int;
                _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv_0);
                _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                _hj_j_2 ^= _hj_i_2 << 16 as libc::c_int;
                _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_2);
                _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_2);
                _ha_hashv_0 ^= _hj_j_2 >> 5 as libc::c_int;
                _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv_0);
                _hj_i_2 ^= _ha_hashv_0 >> 3 as libc::c_int;
                _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv_0);
                _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                _hj_j_2 ^= _hj_i_2 << 10 as libc::c_int;
                _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_2);
                _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_2);
                _ha_hashv_0 ^= _hj_j_2 >> 15 as libc::c_int;
                (*callee).hh.hashv = _ha_hashv_0;
                (*callee)
                    .hh
                    .key = &mut *((*callee).loc_str)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut libc::c_char
                    as *mut libc::c_void;
                (*callee).hh.keylen = _uthash_hastr_keylen_0;
                if ((*prev_caller).callees).is_null() {
                    (*callee).hh.next = 0 as *mut libc::c_void;
                    (*callee).hh.prev = 0 as *mut libc::c_void;
                    (*callee)
                        .hh
                        .tbl = malloc(
                        ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                    ) as *mut UT_hash_table;
                    if ((*callee).hh.tbl).is_null() {
                        exit(-(1 as libc::c_int));
                    } else {
                        memset(
                            (*callee).hh.tbl as *mut libc::c_void,
                            '\0' as i32,
                            ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                        );
                        (*(*callee).hh.tbl).tail = &mut (*callee).hh;
                        (*(*callee).hh.tbl).num_buckets = 32 as libc::c_uint;
                        (*(*callee).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
                        (*(*callee).hh.tbl)
                            .hho = (&mut (*callee).hh as *mut UT_hash_handle
                            as *mut libc::c_char)
                            .offset_from(callee as *mut libc::c_char) as libc::c_long;
                        (*(*callee).hh.tbl)
                            .buckets = malloc(
                            (32 as libc::c_uint as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                                ),
                        ) as *mut UT_hash_bucket;
                        (*(*callee).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
                        if ((*(*callee).hh.tbl).buckets).is_null() {
                            exit(-(1 as libc::c_int));
                        } else {
                            memset(
                                (*(*callee).hh.tbl).buckets as *mut libc::c_void,
                                '\0' as i32,
                                (32 as libc::c_uint as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                                    ),
                            );
                        }
                    }
                    (*prev_caller).callees = callee;
                } else {
                    (*callee).hh.tbl = (*(*prev_caller).callees).hh.tbl;
                    (*callee).hh.next = 0 as *mut libc::c_void;
                    (*callee)
                        .hh
                        .prev = ((*(*(*prev_caller).callees).hh.tbl).tail
                        as *mut libc::c_char)
                        .offset(-((*(*(*prev_caller).callees).hh.tbl).hho as isize))
                        as *mut libc::c_void;
                    (*(*(*(*prev_caller).callees).hh.tbl).tail)
                        .next = callee as *mut libc::c_void;
                    (*(*(*prev_caller).callees).hh.tbl).tail = &mut (*callee).hh;
                }
                let mut _ha_bkt_0: libc::c_uint = 0;
                (*(*(*prev_caller).callees).hh.tbl)
                    .num_items = ((*(*(*prev_caller).callees).hh.tbl).num_items)
                    .wrapping_add(1);
                (*(*(*prev_caller).callees).hh.tbl).num_items;
                _ha_bkt_0 = _ha_hashv_0
                    & ((*(*(*prev_caller).callees).hh.tbl).num_buckets)
                        .wrapping_sub(1 as libc::c_uint);
                let mut _ha_head_0: *mut UT_hash_bucket = &mut *((*(*(*prev_caller)
                    .callees)
                    .hh
                    .tbl)
                    .buckets)
                    .offset(_ha_bkt_0 as isize) as *mut UT_hash_bucket;
                (*_ha_head_0).count = ((*_ha_head_0).count).wrapping_add(1);
                (*_ha_head_0).count;
                (*callee).hh.hh_next = (*_ha_head_0).hh_head;
                (*callee).hh.hh_prev = 0 as *mut UT_hash_handle;
                if !((*_ha_head_0).hh_head).is_null() {
                    (*(*_ha_head_0).hh_head).hh_prev = &mut (*callee).hh;
                }
                (*_ha_head_0).hh_head = &mut (*callee).hh;
                if (*_ha_head_0).count
                    >= ((*_ha_head_0).expand_mult)
                        .wrapping_add(1 as libc::c_uint)
                        .wrapping_mul(10 as libc::c_uint)
                    && (*(*callee).hh.tbl).noexpand == 0
                {
                    let mut _he_bkt_0: libc::c_uint = 0;
                    let mut _he_bkt_i_0: libc::c_uint = 0;
                    let mut _he_thh_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
                    let mut _he_hh_nxt_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
                    let mut _he_new_buckets_0: *mut UT_hash_bucket = 0
                        as *mut UT_hash_bucket;
                    let mut _he_newbkt_0: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
                    _he_new_buckets_0 = malloc(
                        (2 as libc::c_ulong)
                            .wrapping_mul(
                                (*(*callee).hh.tbl).num_buckets as libc::c_ulong,
                            )
                            .wrapping_mul(
                                ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                            ),
                    ) as *mut UT_hash_bucket;
                    if _he_new_buckets_0.is_null() {
                        exit(-(1 as libc::c_int));
                    } else {
                        memset(
                            _he_new_buckets_0 as *mut libc::c_void,
                            '\0' as i32,
                            (2 as libc::c_ulong)
                                .wrapping_mul(
                                    (*(*callee).hh.tbl).num_buckets as libc::c_ulong,
                                )
                                .wrapping_mul(
                                    ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                                ),
                        );
                        (*(*callee).hh.tbl)
                            .ideal_chain_maxlen = ((*(*callee).hh.tbl).num_items
                            >> ((*(*callee).hh.tbl).log2_num_buckets)
                                .wrapping_add(1 as libc::c_uint))
                            .wrapping_add(
                                (if (*(*callee).hh.tbl).num_items
                                    & ((*(*callee).hh.tbl).num_buckets)
                                        .wrapping_mul(2 as libc::c_uint)
                                        .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                                {
                                    1 as libc::c_uint
                                } else {
                                    0 as libc::c_uint
                                }),
                            );
                        (*(*callee).hh.tbl)
                            .nonideal_items = 0 as libc::c_int as libc::c_uint;
                        _he_bkt_i_0 = 0 as libc::c_int as libc::c_uint;
                        while _he_bkt_i_0 < (*(*callee).hh.tbl).num_buckets {
                            _he_thh_0 = (*((*(*callee).hh.tbl).buckets)
                                .offset(_he_bkt_i_0 as isize))
                                .hh_head;
                            while !_he_thh_0.is_null() {
                                _he_hh_nxt_0 = (*_he_thh_0).hh_next;
                                _he_bkt_0 = (*_he_thh_0).hashv
                                    & ((*(*callee).hh.tbl).num_buckets)
                                        .wrapping_mul(2 as libc::c_uint)
                                        .wrapping_sub(1 as libc::c_uint);
                                _he_newbkt_0 = &mut *_he_new_buckets_0
                                    .offset(_he_bkt_0 as isize) as *mut UT_hash_bucket;
                                (*_he_newbkt_0)
                                    .count = ((*_he_newbkt_0).count).wrapping_add(1);
                                if (*_he_newbkt_0).count
                                    > (*(*callee).hh.tbl).ideal_chain_maxlen
                                {
                                    (*(*callee).hh.tbl)
                                        .nonideal_items = ((*(*callee).hh.tbl).nonideal_items)
                                        .wrapping_add(1);
                                    (*(*callee).hh.tbl).nonideal_items;
                                    (*_he_newbkt_0)
                                        .expand_mult = ((*_he_newbkt_0).count)
                                        .wrapping_div((*(*callee).hh.tbl).ideal_chain_maxlen);
                                }
                                (*_he_thh_0).hh_prev = 0 as *mut UT_hash_handle;
                                (*_he_thh_0).hh_next = (*_he_newbkt_0).hh_head;
                                if !((*_he_newbkt_0).hh_head).is_null() {
                                    (*(*_he_newbkt_0).hh_head).hh_prev = _he_thh_0;
                                }
                                (*_he_newbkt_0).hh_head = _he_thh_0;
                                _he_thh_0 = _he_hh_nxt_0;
                            }
                            _he_bkt_i_0 = _he_bkt_i_0.wrapping_add(1);
                            _he_bkt_i_0;
                        }
                        free((*(*callee).hh.tbl).buckets as *mut libc::c_void);
                        (*(*callee).hh.tbl)
                            .num_buckets = ((*(*callee).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint);
                        (*(*callee).hh.tbl)
                            .log2_num_buckets = ((*(*callee).hh.tbl).log2_num_buckets)
                            .wrapping_add(1);
                        (*(*callee).hh.tbl).log2_num_buckets;
                        (*(*callee).hh.tbl).buckets = _he_new_buckets_0;
                        (*(*callee).hh.tbl)
                            .ineff_expands = if (*(*callee).hh.tbl).nonideal_items
                            > (*(*callee).hh.tbl).num_items >> 1 as libc::c_int
                        {
                            ((*(*callee).hh.tbl).ineff_expands)
                                .wrapping_add(1 as libc::c_uint)
                        } else {
                            0 as libc::c_uint
                        };
                        if (*(*callee).hh.tbl).ineff_expands > 1 as libc::c_uint {
                            (*(*callee).hh.tbl)
                                .noexpand = 1 as libc::c_int as libc::c_uint;
                        }
                    }
                }
            }
            (*callee)
                .inclusive = ((*callee).inclusive as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as uint64_t as uint64_t;
            if i >= (*udata).prev_len
                || strcmp(
                    ((*udata).self_str[i as usize]).as_mut_ptr(),
                    ((*udata).prev_str[i as usize]).as_mut_ptr(),
                ) != 0 as libc::c_int
            {
                (*callee)
                    .count = ((*callee).count as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as uint64_t
                    as uint64_t;
            }
        }
        prev_caller = caller;
        i -= 1;
        i;
    }
}
unsafe extern "C" fn callgrind_dump(mut udata: *mut callgrind_udata_t) {
    let mut caller: *mut callgrind_caller_t = 0 as *mut callgrind_caller_t;
    let mut caller_tmp: *mut callgrind_caller_t = 0 as *mut callgrind_caller_t;
    let mut callee: *mut callgrind_callee_t = 0 as *mut callgrind_callee_t;
    let mut callee_tmp: *mut callgrind_callee_t = 0 as *mut callgrind_callee_t;
    fprintf(
        (*udata).fout,
        b"# callgrind format\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf((*udata).fout, b"version: 1\n\0" as *const u8 as *const libc::c_char);
    fprintf((*udata).fout, b"creator: phpspy\n\0" as *const u8 as *const libc::c_char);
    fprintf((*udata).fout, b"events: Samples\n\0" as *const u8 as *const libc::c_char);
    let mut _hs_i: libc::c_uint = 0;
    let mut _hs_looping: libc::c_uint = 0;
    let mut _hs_nmerges: libc::c_uint = 0;
    let mut _hs_insize: libc::c_uint = 0;
    let mut _hs_psize: libc::c_uint = 0;
    let mut _hs_qsize: libc::c_uint = 0;
    let mut _hs_p: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _hs_q: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _hs_e: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _hs_list: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _hs_tail: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    if !((*udata).callers).is_null() {
        _hs_insize = 1 as libc::c_int as libc::c_uint;
        _hs_looping = 1 as libc::c_int as libc::c_uint;
        _hs_list = &mut (*(*udata).callers).hh;
        while _hs_looping != 0 as libc::c_uint {
            _hs_p = _hs_list;
            _hs_list = 0 as *mut UT_hash_handle;
            _hs_tail = 0 as *mut UT_hash_handle;
            _hs_nmerges = 0 as libc::c_int as libc::c_uint;
            while !_hs_p.is_null() {
                _hs_nmerges = _hs_nmerges.wrapping_add(1);
                _hs_nmerges;
                _hs_q = _hs_p;
                _hs_psize = 0 as libc::c_int as libc::c_uint;
                _hs_i = 0 as libc::c_int as libc::c_uint;
                while _hs_i < _hs_insize {
                    _hs_psize = _hs_psize.wrapping_add(1);
                    _hs_psize;
                    _hs_q = if !((*_hs_q).next).is_null() {
                        ((*_hs_q).next as *mut libc::c_char)
                            .offset((*(*(*udata).callers).hh.tbl).hho as isize)
                            as *mut UT_hash_handle
                    } else {
                        0 as *mut UT_hash_handle
                    };
                    if _hs_q.is_null() {
                        break;
                    }
                    _hs_i = _hs_i.wrapping_add(1);
                    _hs_i;
                }
                _hs_qsize = _hs_insize;
                while _hs_psize != 0 as libc::c_uint
                    || _hs_qsize != 0 as libc::c_uint && !_hs_q.is_null()
                {
                    if _hs_psize == 0 as libc::c_uint {
                        _hs_e = _hs_q;
                        _hs_q = if !((*_hs_q).next).is_null() {
                            ((*_hs_q).next as *mut libc::c_char)
                                .offset((*(*(*udata).callers).hh.tbl).hho as isize)
                                as *mut UT_hash_handle
                        } else {
                            0 as *mut UT_hash_handle
                        };
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                        _hs_qsize;
                    } else if _hs_qsize == 0 as libc::c_uint || _hs_q.is_null() {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = if !((*_hs_p).next).is_null() {
                                ((*_hs_p).next as *mut libc::c_char)
                                    .offset((*(*(*udata).callers).hh.tbl).hho as isize)
                                    as *mut UT_hash_handle
                            } else {
                                0 as *mut UT_hash_handle
                            };
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                        _hs_psize;
                    } else if callgrind_sort_callers(
                        (_hs_p as *mut libc::c_char)
                            .offset(-((*(*(*udata).callers).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut callgrind_caller_t,
                        (_hs_q as *mut libc::c_char)
                            .offset(-((*(*(*udata).callers).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut callgrind_caller_t,
                    ) <= 0 as libc::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = if !((*_hs_p).next).is_null() {
                                ((*_hs_p).next as *mut libc::c_char)
                                    .offset((*(*(*udata).callers).hh.tbl).hho as isize)
                                    as *mut UT_hash_handle
                            } else {
                                0 as *mut UT_hash_handle
                            };
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                        _hs_psize;
                    } else {
                        _hs_e = _hs_q;
                        _hs_q = if !((*_hs_q).next).is_null() {
                            ((*_hs_q).next as *mut libc::c_char)
                                .offset((*(*(*udata).callers).hh.tbl).hho as isize)
                                as *mut UT_hash_handle
                        } else {
                            0 as *mut UT_hash_handle
                        };
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                        _hs_qsize;
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail)
                            .next = if !_hs_e.is_null() {
                            (_hs_e as *mut libc::c_char)
                                .offset(-((*(*(*udata).callers).hh.tbl).hho as isize))
                                as *mut libc::c_void
                        } else {
                            0 as *mut libc::c_void
                        };
                    } else {
                        _hs_list = _hs_e;
                    }
                    if !_hs_e.is_null() {
                        (*_hs_e)
                            .prev = if !_hs_tail.is_null() {
                            (_hs_tail as *mut libc::c_char)
                                .offset(-((*(*(*udata).callers).hh.tbl).hho as isize))
                                as *mut libc::c_void
                        } else {
                            0 as *mut libc::c_void
                        };
                    }
                    _hs_tail = _hs_e;
                }
                _hs_p = _hs_q;
            }
            if !_hs_tail.is_null() {
                (*_hs_tail).next = 0 as *mut libc::c_void;
            }
            if _hs_nmerges <= 1 as libc::c_uint {
                _hs_looping = 0 as libc::c_int as libc::c_uint;
                (*(*(*udata).callers).hh.tbl).tail = _hs_tail;
                (*udata)
                    .callers = (_hs_list as *mut libc::c_char)
                    .offset(-((*(*(*udata).callers).hh.tbl).hho as isize))
                    as *mut libc::c_void as *mut callgrind_caller_t;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as libc::c_uint);
        }
    }
    caller = (*udata).callers;
    caller_tmp = (if !((*udata).callers).is_null() {
        (*(*udata).callers).hh.next
    } else {
        0 as *mut libc::c_void
    }) as *mut callgrind_caller_t;
    while !caller.is_null() {
        fprintf((*udata).fout, b"\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            (*udata).fout,
            b"fl=%.*s\n\0" as *const u8 as *const libc::c_char,
            (*caller).loc.file_len as libc::c_int,
            ((*caller).loc.file).as_mut_ptr(),
        );
        fprintf(
            (*udata).fout,
            b"fn=%.*s%s%.*s\n\0" as *const u8 as *const libc::c_char,
            (*caller).loc.class_len as libc::c_int,
            ((*caller).loc.class).as_mut_ptr(),
            if (*caller).loc.class_len > 0 as libc::c_int as libc::c_ulong {
                b"::\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            (*caller).loc.func_len as libc::c_int,
            ((*caller).loc.func).as_mut_ptr(),
        );
        fprintf(
            (*udata).fout,
            b"%d %ld\n\0" as *const u8 as *const libc::c_char,
            (*caller).loc.lineno,
            (*caller).exclusive,
        );
        let mut _hs_i_0: libc::c_uint = 0;
        let mut _hs_looping_0: libc::c_uint = 0;
        let mut _hs_nmerges_0: libc::c_uint = 0;
        let mut _hs_insize_0: libc::c_uint = 0;
        let mut _hs_psize_0: libc::c_uint = 0;
        let mut _hs_qsize_0: libc::c_uint = 0;
        let mut _hs_p_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _hs_q_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _hs_e_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _hs_list_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _hs_tail_0: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        if !((*caller).callees).is_null() {
            _hs_insize_0 = 1 as libc::c_int as libc::c_uint;
            _hs_looping_0 = 1 as libc::c_int as libc::c_uint;
            _hs_list_0 = &mut (*(*caller).callees).hh;
            while _hs_looping_0 != 0 as libc::c_uint {
                _hs_p_0 = _hs_list_0;
                _hs_list_0 = 0 as *mut UT_hash_handle;
                _hs_tail_0 = 0 as *mut UT_hash_handle;
                _hs_nmerges_0 = 0 as libc::c_int as libc::c_uint;
                while !_hs_p_0.is_null() {
                    _hs_nmerges_0 = _hs_nmerges_0.wrapping_add(1);
                    _hs_nmerges_0;
                    _hs_q_0 = _hs_p_0;
                    _hs_psize_0 = 0 as libc::c_int as libc::c_uint;
                    _hs_i_0 = 0 as libc::c_int as libc::c_uint;
                    while _hs_i_0 < _hs_insize_0 {
                        _hs_psize_0 = _hs_psize_0.wrapping_add(1);
                        _hs_psize_0;
                        _hs_q_0 = if !((*_hs_q_0).next).is_null() {
                            ((*_hs_q_0).next as *mut libc::c_char)
                                .offset((*(*(*caller).callees).hh.tbl).hho as isize)
                                as *mut UT_hash_handle
                        } else {
                            0 as *mut UT_hash_handle
                        };
                        if _hs_q_0.is_null() {
                            break;
                        }
                        _hs_i_0 = _hs_i_0.wrapping_add(1);
                        _hs_i_0;
                    }
                    _hs_qsize_0 = _hs_insize_0;
                    while _hs_psize_0 != 0 as libc::c_uint
                        || _hs_qsize_0 != 0 as libc::c_uint && !_hs_q_0.is_null()
                    {
                        if _hs_psize_0 == 0 as libc::c_uint {
                            _hs_e_0 = _hs_q_0;
                            _hs_q_0 = if !((*_hs_q_0).next).is_null() {
                                ((*_hs_q_0).next as *mut libc::c_char)
                                    .offset((*(*(*caller).callees).hh.tbl).hho as isize)
                                    as *mut UT_hash_handle
                            } else {
                                0 as *mut UT_hash_handle
                            };
                            _hs_qsize_0 = _hs_qsize_0.wrapping_sub(1);
                            _hs_qsize_0;
                        } else if _hs_qsize_0 == 0 as libc::c_uint || _hs_q_0.is_null() {
                            _hs_e_0 = _hs_p_0;
                            if !_hs_p_0.is_null() {
                                _hs_p_0 = if !((*_hs_p_0).next).is_null() {
                                    ((*_hs_p_0).next as *mut libc::c_char)
                                        .offset((*(*(*caller).callees).hh.tbl).hho as isize)
                                        as *mut UT_hash_handle
                                } else {
                                    0 as *mut UT_hash_handle
                                };
                            }
                            _hs_psize_0 = _hs_psize_0.wrapping_sub(1);
                            _hs_psize_0;
                        } else if callgrind_sort_callees(
                            (_hs_p_0 as *mut libc::c_char)
                                .offset(-((*(*(*caller).callees).hh.tbl).hho as isize))
                                as *mut libc::c_void as *mut callgrind_callee_t,
                            (_hs_q_0 as *mut libc::c_char)
                                .offset(-((*(*(*caller).callees).hh.tbl).hho as isize))
                                as *mut libc::c_void as *mut callgrind_callee_t,
                        ) <= 0 as libc::c_int
                        {
                            _hs_e_0 = _hs_p_0;
                            if !_hs_p_0.is_null() {
                                _hs_p_0 = if !((*_hs_p_0).next).is_null() {
                                    ((*_hs_p_0).next as *mut libc::c_char)
                                        .offset((*(*(*caller).callees).hh.tbl).hho as isize)
                                        as *mut UT_hash_handle
                                } else {
                                    0 as *mut UT_hash_handle
                                };
                            }
                            _hs_psize_0 = _hs_psize_0.wrapping_sub(1);
                            _hs_psize_0;
                        } else {
                            _hs_e_0 = _hs_q_0;
                            _hs_q_0 = if !((*_hs_q_0).next).is_null() {
                                ((*_hs_q_0).next as *mut libc::c_char)
                                    .offset((*(*(*caller).callees).hh.tbl).hho as isize)
                                    as *mut UT_hash_handle
                            } else {
                                0 as *mut UT_hash_handle
                            };
                            _hs_qsize_0 = _hs_qsize_0.wrapping_sub(1);
                            _hs_qsize_0;
                        }
                        if !_hs_tail_0.is_null() {
                            (*_hs_tail_0)
                                .next = if !_hs_e_0.is_null() {
                                (_hs_e_0 as *mut libc::c_char)
                                    .offset(-((*(*(*caller).callees).hh.tbl).hho as isize))
                                    as *mut libc::c_void
                            } else {
                                0 as *mut libc::c_void
                            };
                        } else {
                            _hs_list_0 = _hs_e_0;
                        }
                        if !_hs_e_0.is_null() {
                            (*_hs_e_0)
                                .prev = if !_hs_tail_0.is_null() {
                                (_hs_tail_0 as *mut libc::c_char)
                                    .offset(-((*(*(*caller).callees).hh.tbl).hho as isize))
                                    as *mut libc::c_void
                            } else {
                                0 as *mut libc::c_void
                            };
                        }
                        _hs_tail_0 = _hs_e_0;
                    }
                    _hs_p_0 = _hs_q_0;
                }
                if !_hs_tail_0.is_null() {
                    (*_hs_tail_0).next = 0 as *mut libc::c_void;
                }
                if _hs_nmerges_0 <= 1 as libc::c_uint {
                    _hs_looping_0 = 0 as libc::c_int as libc::c_uint;
                    (*(*(*caller).callees).hh.tbl).tail = _hs_tail_0;
                    (*caller)
                        .callees = (_hs_list_0 as *mut libc::c_char)
                        .offset(-((*(*(*caller).callees).hh.tbl).hho as isize))
                        as *mut libc::c_void as *mut callgrind_callee_t;
                }
                _hs_insize_0 = _hs_insize_0.wrapping_mul(2 as libc::c_uint);
            }
        }
        callee = (*caller).callees;
        callee_tmp = (if !((*caller).callees).is_null() {
            (*(*caller).callees).hh.next
        } else {
            0 as *mut libc::c_void
        }) as *mut callgrind_callee_t;
        while !callee.is_null() {
            fprintf((*udata).fout, b"\n\0" as *const u8 as *const libc::c_char);
            fprintf(
                (*udata).fout,
                b"cfl=%.*s\n\0" as *const u8 as *const libc::c_char,
                (*callee).loc.file_len as libc::c_int,
                ((*callee).loc.file).as_mut_ptr(),
            );
            fprintf(
                (*udata).fout,
                b"cfn=%.*s%s%.*s\n\0" as *const u8 as *const libc::c_char,
                (*callee).loc.class_len as libc::c_int,
                ((*callee).loc.class).as_mut_ptr(),
                if (*callee).loc.class_len > 0 as libc::c_int as libc::c_ulong {
                    b"::\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                (*callee).loc.func_len as libc::c_int,
                ((*callee).loc.func).as_mut_ptr(),
            );
            fprintf(
                (*udata).fout,
                b"calls=%ld %d\n\0" as *const u8 as *const libc::c_char,
                (*callee).count,
                (*callee).loc.lineno,
            );
            fprintf(
                (*udata).fout,
                b"%d %ld\n\0" as *const u8 as *const libc::c_char,
                (*caller).loc.lineno,
                (*callee).inclusive,
            );
            callee = callee_tmp;
            callee_tmp = (if !callee_tmp.is_null() {
                (*callee_tmp).hh.next
            } else {
                0 as *mut libc::c_void
            }) as *mut callgrind_callee_t;
        }
        caller = caller_tmp;
        caller_tmp = (if !caller_tmp.is_null() {
            (*caller_tmp).hh.next
        } else {
            0 as *mut libc::c_void
        }) as *mut callgrind_caller_t;
    }
}
unsafe extern "C" fn callgrind_free(mut udata: *mut callgrind_udata_t) {
    let mut caller: *mut callgrind_caller_t = 0 as *mut callgrind_caller_t;
    let mut caller_tmp: *mut callgrind_caller_t = 0 as *mut callgrind_caller_t;
    let mut callee: *mut callgrind_callee_t = 0 as *mut callgrind_callee_t;
    let mut callee_tmp: *mut callgrind_callee_t = 0 as *mut callgrind_callee_t;
    caller = (*udata).callers;
    caller_tmp = (if !((*udata).callers).is_null() {
        (*(*udata).callers).hh.next
    } else {
        0 as *mut libc::c_void
    }) as *mut callgrind_caller_t;
    while !caller.is_null() {
        callee = (*caller).callees;
        callee_tmp = (if !((*caller).callees).is_null() {
            (*(*caller).callees).hh.next
        } else {
            0 as *mut libc::c_void
        }) as *mut callgrind_callee_t;
        while !callee.is_null() {
            let mut _hd_hh_del: *mut UT_hash_handle = &mut (*callee).hh;
            if ((*_hd_hh_del).prev).is_null() && ((*_hd_hh_del).next).is_null() {
                free((*(*(*caller).callees).hh.tbl).buckets as *mut libc::c_void);
                free((*(*caller).callees).hh.tbl as *mut libc::c_void);
                (*caller).callees = 0 as *mut callgrind_callee_t;
            } else {
                let mut _hd_bkt: libc::c_uint = 0;
                if _hd_hh_del == (*(*(*caller).callees).hh.tbl).tail {
                    (*(*(*caller).callees).hh.tbl)
                        .tail = ((*_hd_hh_del).prev as *mut libc::c_char)
                        .offset((*(*(*caller).callees).hh.tbl).hho as isize)
                        as *mut UT_hash_handle;
                }
                if !((*_hd_hh_del).prev).is_null() {
                    let ref mut fresh0 = (*(((*_hd_hh_del).prev as *mut libc::c_char)
                        .offset((*(*(*caller).callees).hh.tbl).hho as isize)
                        as *mut UT_hash_handle))
                        .next;
                    *fresh0 = (*_hd_hh_del).next;
                } else {
                    (*caller).callees = (*_hd_hh_del).next as *mut callgrind_callee_t;
                }
                if !((*_hd_hh_del).next).is_null() {
                    let ref mut fresh1 = (*(((*_hd_hh_del).next as *mut libc::c_char)
                        .offset((*(*(*caller).callees).hh.tbl).hho as isize)
                        as *mut UT_hash_handle))
                        .prev;
                    *fresh1 = (*_hd_hh_del).prev;
                }
                _hd_bkt = (*_hd_hh_del).hashv
                    & ((*(*(*caller).callees).hh.tbl).num_buckets)
                        .wrapping_sub(1 as libc::c_uint);
                let mut _hd_head: *mut UT_hash_bucket = &mut *((*(*(*caller).callees)
                    .hh
                    .tbl)
                    .buckets)
                    .offset(_hd_bkt as isize) as *mut UT_hash_bucket;
                (*_hd_head).count = ((*_hd_head).count).wrapping_sub(1);
                (*_hd_head).count;
                if (*_hd_head).hh_head == _hd_hh_del {
                    (*_hd_head).hh_head = (*_hd_hh_del).hh_next;
                }
                if !((*_hd_hh_del).hh_prev).is_null() {
                    (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
                }
                if !((*_hd_hh_del).hh_next).is_null() {
                    (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
                }
                (*(*(*caller).callees).hh.tbl)
                    .num_items = ((*(*(*caller).callees).hh.tbl).num_items)
                    .wrapping_sub(1);
                (*(*(*caller).callees).hh.tbl).num_items;
            }
            free(callee as *mut libc::c_void);
            callee = callee_tmp;
            callee_tmp = (if !callee_tmp.is_null() {
                (*callee_tmp).hh.next
            } else {
                0 as *mut libc::c_void
            }) as *mut callgrind_callee_t;
        }
        let mut _hd_hh_del_0: *mut UT_hash_handle = &mut (*caller).hh;
        if ((*_hd_hh_del_0).prev).is_null() && ((*_hd_hh_del_0).next).is_null() {
            free((*(*(*udata).callers).hh.tbl).buckets as *mut libc::c_void);
            free((*(*udata).callers).hh.tbl as *mut libc::c_void);
            (*udata).callers = 0 as *mut callgrind_caller_t;
        } else {
            let mut _hd_bkt_0: libc::c_uint = 0;
            if _hd_hh_del_0 == (*(*(*udata).callers).hh.tbl).tail {
                (*(*(*udata).callers).hh.tbl)
                    .tail = ((*_hd_hh_del_0).prev as *mut libc::c_char)
                    .offset((*(*(*udata).callers).hh.tbl).hho as isize)
                    as *mut UT_hash_handle;
            }
            if !((*_hd_hh_del_0).prev).is_null() {
                let ref mut fresh2 = (*(((*_hd_hh_del_0).prev as *mut libc::c_char)
                    .offset((*(*(*udata).callers).hh.tbl).hho as isize)
                    as *mut UT_hash_handle))
                    .next;
                *fresh2 = (*_hd_hh_del_0).next;
            } else {
                (*udata).callers = (*_hd_hh_del_0).next as *mut callgrind_caller_t;
            }
            if !((*_hd_hh_del_0).next).is_null() {
                let ref mut fresh3 = (*(((*_hd_hh_del_0).next as *mut libc::c_char)
                    .offset((*(*(*udata).callers).hh.tbl).hho as isize)
                    as *mut UT_hash_handle))
                    .prev;
                *fresh3 = (*_hd_hh_del_0).prev;
            }
            _hd_bkt_0 = (*_hd_hh_del_0).hashv
                & ((*(*(*udata).callers).hh.tbl).num_buckets)
                    .wrapping_sub(1 as libc::c_uint);
            let mut _hd_head_0: *mut UT_hash_bucket = &mut *((*(*(*udata).callers)
                .hh
                .tbl)
                .buckets)
                .offset(_hd_bkt_0 as isize) as *mut UT_hash_bucket;
            (*_hd_head_0).count = ((*_hd_head_0).count).wrapping_sub(1);
            (*_hd_head_0).count;
            if (*_hd_head_0).hh_head == _hd_hh_del_0 {
                (*_hd_head_0).hh_head = (*_hd_hh_del_0).hh_next;
            }
            if !((*_hd_hh_del_0).hh_prev).is_null() {
                (*(*_hd_hh_del_0).hh_prev).hh_next = (*_hd_hh_del_0).hh_next;
            }
            if !((*_hd_hh_del_0).hh_next).is_null() {
                (*(*_hd_hh_del_0).hh_next).hh_prev = (*_hd_hh_del_0).hh_prev;
            }
            (*(*(*udata).callers).hh.tbl)
                .num_items = ((*(*(*udata).callers).hh.tbl).num_items).wrapping_sub(1);
            (*(*(*udata).callers).hh.tbl).num_items;
        }
        free(caller as *mut libc::c_void);
        caller = caller_tmp;
        caller_tmp = (if !caller_tmp.is_null() {
            (*caller_tmp).hh.next
        } else {
            0 as *mut libc::c_void
        }) as *mut callgrind_caller_t;
    }
}
unsafe extern "C" fn callgrind_sort_callers(
    mut a: *mut callgrind_caller_t,
    mut b: *mut callgrind_caller_t,
) -> libc::c_int {
    return strcmp(((*a).loc_str).as_mut_ptr(), ((*b).loc_str).as_mut_ptr());
}
unsafe extern "C" fn callgrind_sort_callees(
    mut a: *mut callgrind_callee_t,
    mut b: *mut callgrind_callee_t,
) -> libc::c_int {
    return strcmp(((*a).loc_str).as_mut_ptr(), ((*b).loc_str).as_mut_ptr());
}
unsafe extern "C" fn callgrind_open(mut fout: *mut *mut FILE) -> libc::c_int {
    let mut tfd: libc::c_int = 0;
    tfd = -(1 as libc::c_int);
    if strcmp(opt_path_output, b"-\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        tfd = dup(1 as libc::c_int);
        *fout = fdopen(tfd, b"w\0" as *const u8 as *const libc::c_char);
    } else {
        *fout = fopen(opt_path_output, b"w\0" as *const u8 as *const libc::c_char);
    }
    if (*fout).is_null() {
        perror(b"fopen\0" as *const u8 as *const libc::c_char);
        if tfd != -(1 as libc::c_int) {
            close(tfd);
        }
        return 1 as libc::c_int;
    }
    setvbuf(
        *fout,
        0 as *mut libc::c_char,
        1 as libc::c_int,
        4096 as libc::c_int as size_t,
    );
    return 0 as libc::c_int;
}
