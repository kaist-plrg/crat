use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn regexec(
        __preg: *const regex_t,
        __String: *const libc::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: libc::c_int,
    ) -> libc::c_int;
    static mut opt_frame_delim: libc::c_char;
    static mut opt_trace_delim: libc::c_char;
    static mut opt_path_output: *mut libc::c_char;
    static mut opt_filter_re: *mut regex_t;
    static mut opt_filter_negate: libc::c_int;
    static mut opt_verbose_fields_pid: libc::c_int;
    static mut opt_verbose_fields_ts: libc::c_int;
    static mut opt_fout_buffer_size: libc::c_int;
    fn log_error(fmt: *const libc::c_char, _: ...);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type va_list = __builtin_va_list;
pub type ptrdiff_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type ssize_t = __ssize_t;
pub type __re_long_size_t = libc::c_ulong;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
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
pub type event_handler_fout_udata_t = event_handler_fout_udata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_handler_fout_udata_s {
    pub fd: libc::c_int,
    pub buf: *mut libc::c_char,
    pub cur: *mut libc::c_char,
    pub buf_size: size_t,
    pub rem: size_t,
}
pub unsafe extern "C" fn event_handler_fout(
    mut context: *mut trace_context_s,
    mut event_type: libc::c_int,
) -> libc::c_int {
    let mut rv: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut frame: *mut trace_frame_t = 0 as *mut trace_frame_t;
    let mut request: *mut trace_request_t = 0 as *mut trace_request_t;
    let mut udata: *mut event_handler_fout_udata_t = 0
        as *mut event_handler_fout_udata_t;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    udata = (*context).event_udata as *mut event_handler_fout_udata_t;
    if udata.is_null() && event_type != 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    len = 0 as libc::c_int as size_t;
    let mut current_block_112: u64;
    match event_type {
        0 => {
            rv = event_handler_fout_open(&mut fd);
            if rv != 0 as libc::c_int {
                return rv;
            }
            udata = calloc(
                1 as libc::c_int as libc::c_ulong,
                ::std::mem::size_of::<event_handler_fout_udata_t>() as libc::c_ulong,
            ) as *mut event_handler_fout_udata_t;
            (*udata).fd = fd;
            (*udata).buf_size = (opt_fout_buffer_size + 1 as libc::c_int) as size_t;
            (*udata).buf = malloc((*udata).buf_size) as *mut libc::c_char;
            (*udata).cur = (*udata).buf;
            (*udata).rem = (*udata).buf_size;
            (*context).event_udata = udata as *mut libc::c_void;
        }
        1 => {
            (*udata).cur = (*udata).buf;
            *((*udata).cur)
                .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            (*udata).rem = (*udata).buf_size;
        }
        2 => {
            frame = &mut (*context).event.frame;
            rv = event_handler_fout_snprintf(
                &mut (*udata).cur as *mut *mut libc::c_char,
                &mut (*udata).rem as *mut size_t,
                &mut len as *mut size_t,
                1 as libc::c_int,
                b"%d %.*s%s%.*s %.*s:%d\0" as *const u8 as *const libc::c_char,
                (*frame).depth,
                (*frame).loc.class_len as libc::c_int,
                ((*frame).loc.class).as_mut_ptr(),
                (if (*frame).loc.class_len > 0 as libc::c_int as libc::c_ulong {
                    b"::\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                }),
                (*frame).loc.func_len as libc::c_int,
                ((*frame).loc.func).as_mut_ptr(),
                (*frame).loc.file_len as libc::c_int,
                ((*frame).loc.file).as_mut_ptr(),
                (*frame).loc.lineno,
            );
            if rv != 0 as libc::c_int {
                return rv;
            }
            rv = event_handler_fout_snprintf(
                &mut (*udata).cur as *mut *mut libc::c_char,
                &mut (*udata).rem as *mut size_t,
                &mut len as *mut size_t,
                0 as libc::c_int,
                b"%c\0" as *const u8 as *const libc::c_char,
                opt_frame_delim as libc::c_int,
            );
            if rv != 0 as libc::c_int {
                return rv;
            }
        }
        3 => {
            rv = event_handler_fout_snprintf(
                &mut (*udata).cur as *mut *mut libc::c_char,
                &mut (*udata).rem as *mut size_t,
                &mut len as *mut size_t,
                1 as libc::c_int,
                b"# varpeek %s@%s = %s\0" as *const u8 as *const libc::c_char,
                ((*(*context).event.varpeek.var).name).as_mut_ptr(),
                ((*(*context).event.varpeek.entry).filename_lineno).as_mut_ptr(),
                (*context).event.varpeek.zval_str,
            );
            if rv != 0 as libc::c_int {
                return rv;
            }
            rv = event_handler_fout_snprintf(
                &mut (*udata).cur as *mut *mut libc::c_char,
                &mut (*udata).rem as *mut size_t,
                &mut len as *mut size_t,
                0 as libc::c_int,
                b"%c\0" as *const u8 as *const libc::c_char,
                opt_frame_delim as libc::c_int,
            );
            if rv != 0 as libc::c_int {
                return rv;
            }
        }
        4 => {
            rv = event_handler_fout_snprintf(
                &mut (*udata).cur as *mut *mut libc::c_char,
                &mut (*udata).rem as *mut size_t,
                &mut len as *mut size_t,
                1 as libc::c_int,
                b"# glopeek %s = %s\0" as *const u8 as *const libc::c_char,
                ((*(*context).event.glopeek.gentry).key).as_mut_ptr(),
                (*context).event.glopeek.zval_str,
            );
            if rv != 0 as libc::c_int {
                return rv;
            }
            rv = event_handler_fout_snprintf(
                &mut (*udata).cur as *mut *mut libc::c_char,
                &mut (*udata).rem as *mut size_t,
                &mut len as *mut size_t,
                0 as libc::c_int,
                b"%c\0" as *const u8 as *const libc::c_char,
                opt_frame_delim as libc::c_int,
            );
            if rv != 0 as libc::c_int {
                return rv;
            }
        }
        5 => {
            request = &mut (*context).event.request;
            rv = event_handler_fout_snprintf(
                &mut (*udata).cur as *mut *mut libc::c_char,
                &mut (*udata).rem as *mut size_t,
                &mut len as *mut size_t,
                1 as libc::c_int,
                b"# uri = %s\0" as *const u8 as *const libc::c_char,
                ((*request).uri).as_mut_ptr(),
            );
            if rv != 0 as libc::c_int {
                return rv;
            }
            rv = event_handler_fout_snprintf(
                &mut (*udata).cur as *mut *mut libc::c_char,
                &mut (*udata).rem as *mut size_t,
                &mut len as *mut size_t,
                0 as libc::c_int,
                b"%c\0" as *const u8 as *const libc::c_char,
                opt_frame_delim as libc::c_int,
            );
            if rv != 0 as libc::c_int {
                return rv;
            }
            rv = event_handler_fout_snprintf(
                &mut (*udata).cur as *mut *mut libc::c_char,
                &mut (*udata).rem as *mut size_t,
                &mut len as *mut size_t,
                1 as libc::c_int,
                b"# path = %s\0" as *const u8 as *const libc::c_char,
                ((*request).path).as_mut_ptr(),
            );
            if rv != 0 as libc::c_int {
                return rv;
            }
            rv = event_handler_fout_snprintf(
                &mut (*udata).cur as *mut *mut libc::c_char,
                &mut (*udata).rem as *mut size_t,
                &mut len as *mut size_t,
                0 as libc::c_int,
                b"%c\0" as *const u8 as *const libc::c_char,
                opt_frame_delim as libc::c_int,
            );
            if rv != 0 as libc::c_int {
                return rv;
            }
            rv = event_handler_fout_snprintf(
                &mut (*udata).cur as *mut *mut libc::c_char,
                &mut (*udata).rem as *mut size_t,
                &mut len as *mut size_t,
                1 as libc::c_int,
                b"# qstring = %s\0" as *const u8 as *const libc::c_char,
                ((*request).qstring).as_mut_ptr(),
            );
            if rv != 0 as libc::c_int {
                return rv;
            }
            rv = event_handler_fout_snprintf(
                &mut (*udata).cur as *mut *mut libc::c_char,
                &mut (*udata).rem as *mut size_t,
                &mut len as *mut size_t,
                0 as libc::c_int,
                b"%c\0" as *const u8 as *const libc::c_char,
                opt_frame_delim as libc::c_int,
            );
            if rv != 0 as libc::c_int {
                return rv;
            }
            rv = event_handler_fout_snprintf(
                &mut (*udata).cur as *mut *mut libc::c_char,
                &mut (*udata).rem as *mut size_t,
                &mut len as *mut size_t,
                1 as libc::c_int,
                b"# cookie = %s\0" as *const u8 as *const libc::c_char,
                ((*request).cookie).as_mut_ptr(),
            );
            if rv != 0 as libc::c_int {
                return rv;
            }
            rv = event_handler_fout_snprintf(
                &mut (*udata).cur as *mut *mut libc::c_char,
                &mut (*udata).rem as *mut size_t,
                &mut len as *mut size_t,
                0 as libc::c_int,
                b"%c\0" as *const u8 as *const libc::c_char,
                opt_frame_delim as libc::c_int,
            );
            if rv != 0 as libc::c_int {
                return rv;
            }
            rv = event_handler_fout_snprintf(
                &mut (*udata).cur as *mut *mut libc::c_char,
                &mut (*udata).rem as *mut size_t,
                &mut len as *mut size_t,
                1 as libc::c_int,
                b"# ts = %f\0" as *const u8 as *const libc::c_char,
                (*request).ts,
            );
            if rv != 0 as libc::c_int {
                return rv;
            }
            rv = event_handler_fout_snprintf(
                &mut (*udata).cur as *mut *mut libc::c_char,
                &mut (*udata).rem as *mut size_t,
                &mut len as *mut size_t,
                0 as libc::c_int,
                b"%c\0" as *const u8 as *const libc::c_char,
                opt_frame_delim as libc::c_int,
            );
            if rv != 0 as libc::c_int {
                return rv;
            }
        }
        6 => {
            rv = event_handler_fout_snprintf(
                &mut (*udata).cur as *mut *mut libc::c_char,
                &mut (*udata).rem as *mut size_t,
                &mut len as *mut size_t,
                1 as libc::c_int,
                b"# mem %lu %lu\0" as *const u8 as *const libc::c_char,
                (*context).event.mem.size,
                (*context).event.mem.peak,
            );
            if rv != 0 as libc::c_int {
                return rv;
            }
            rv = event_handler_fout_snprintf(
                &mut (*udata).cur as *mut *mut libc::c_char,
                &mut (*udata).rem as *mut size_t,
                &mut len as *mut size_t,
                0 as libc::c_int,
                b"%c\0" as *const u8 as *const libc::c_char,
                opt_frame_delim as libc::c_int,
            );
            if rv != 0 as libc::c_int {
                return rv;
            }
        }
        7 => {
            if !((*udata).cur == (*udata).buf) {
                if !opt_filter_re.is_null() {
                    rv = regexec(
                        opt_filter_re,
                        (*udata).buf,
                        0 as libc::c_int as size_t,
                        0 as *mut regmatch_t,
                        0 as libc::c_int,
                    );
                    if opt_filter_negate == 0 as libc::c_int && rv != 0 as libc::c_int {
                        current_block_112 = 18002345992382212654;
                    } else if opt_filter_negate != 0 as libc::c_int
                        && rv == 0 as libc::c_int
                    {
                        current_block_112 = 18002345992382212654;
                    } else {
                        current_block_112 = 2750570471926810434;
                    }
                } else {
                    current_block_112 = 2750570471926810434;
                }
                match current_block_112 {
                    18002345992382212654 => {}
                    _ => {
                        if opt_verbose_fields_ts != 0 {
                            gettimeofday(&mut tv, 0 as *mut libc::c_void);
                            rv = event_handler_fout_snprintf(
                                &mut (*udata).cur as *mut *mut libc::c_char,
                                &mut (*udata).rem as *mut size_t,
                                &mut len as *mut size_t,
                                1 as libc::c_int,
                                b"# trace_ts = %f\0" as *const u8 as *const libc::c_char,
                                tv.tv_sec as libc::c_double
                                    + tv.tv_usec as libc::c_double / 1000000.0f64,
                            );
                            rv != 0 as libc::c_int;
                            rv = event_handler_fout_snprintf(
                                &mut (*udata).cur as *mut *mut libc::c_char,
                                &mut (*udata).rem as *mut size_t,
                                &mut len as *mut size_t,
                                0 as libc::c_int,
                                b"%c\0" as *const u8 as *const libc::c_char,
                                opt_frame_delim as libc::c_int,
                            );
                            rv != 0 as libc::c_int;
                        }
                        if opt_verbose_fields_pid != 0 {
                            rv = event_handler_fout_snprintf(
                                &mut (*udata).cur as *mut *mut libc::c_char,
                                &mut (*udata).rem as *mut size_t,
                                &mut len as *mut size_t,
                                1 as libc::c_int,
                                b"# pid = %d\0" as *const u8 as *const libc::c_char,
                                (*context).target.pid,
                            );
                            rv != 0 as libc::c_int;
                            rv = event_handler_fout_snprintf(
                                &mut (*udata).cur as *mut *mut libc::c_char,
                                &mut (*udata).rem as *mut size_t,
                                &mut len as *mut size_t,
                                0 as libc::c_int,
                                b"%c\0" as *const u8 as *const libc::c_char,
                                opt_frame_delim as libc::c_int,
                            );
                            rv != 0 as libc::c_int;
                        }
                        rv = event_handler_fout_snprintf(
                            &mut (*udata).cur as *mut *mut libc::c_char,
                            &mut (*udata).rem as *mut size_t,
                            &mut len as *mut size_t,
                            0 as libc::c_int,
                            b"%c\0" as *const u8 as *const libc::c_char,
                            opt_trace_delim as libc::c_int,
                        );
                        rv != 0 as libc::c_int;
                        rv = event_handler_fout_write(udata);
                        if rv != 0 as libc::c_int {
                            return rv;
                        }
                    }
                }
            }
        }
        9 => {
            close((*udata).fd);
            free((*udata).buf as *mut libc::c_void);
            free(udata as *mut libc::c_void);
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn event_handler_fout_write(
    mut udata: *mut event_handler_fout_udata_t,
) -> libc::c_int {
    let mut write_len: ssize_t = 0;
    write_len = ((*udata).cur).offset_from((*udata).buf) as libc::c_long;
    if !(write_len < 1 as libc::c_int as libc::c_long) {
        if write((*udata).fd, (*udata).buf as *const libc::c_void, write_len as size_t)
            != write_len
        {
            log_error(
                b"event_handler_fout: Write failed (%s)\n\0" as *const u8
                    as *const libc::c_char,
                if *__errno_location() != 0 as libc::c_int {
                    strerror(*__errno_location()) as *const libc::c_char
                } else {
                    b"partial\0" as *const u8 as *const libc::c_char
                },
            );
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn event_handler_fout_snprintf(
    mut s: *mut *mut libc::c_char,
    mut n: *mut size_t,
    mut ret_len: *mut size_t,
    mut repl_delim: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut vl: ::std::ffi::VaListImpl;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    vl = args.clone();
    len = vsnprintf(*s, *n, fmt, vl.as_va_list());
    if len < 0 as libc::c_int || len as size_t >= *n {
        log_error(
            b"event_handler_fout_snprintf: Not enough space in buffer; truncating\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int | 4 as libc::c_int;
    }
    if repl_delim != 0 {
        i = 0 as libc::c_int;
        while i < len {
            c = (*s).offset(i as isize);
            if *c as libc::c_int == opt_trace_delim as libc::c_int
                || *c as libc::c_int == opt_frame_delim as libc::c_int
            {
                *c = '?' as i32 as libc::c_char;
            }
            i += 1;
            i;
        }
    }
    *s = (*s).offset(len as isize);
    *n = (*n as libc::c_ulong).wrapping_sub(len as libc::c_ulong) as size_t as size_t;
    *ret_len = len as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn event_handler_fout_open(mut fd: *mut libc::c_int) -> libc::c_int {
    let mut tfd: libc::c_int = 0;
    if strcmp(opt_path_output, b"-\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        tfd = dup(1 as libc::c_int);
        if tfd < 0 as libc::c_int {
            perror(
                b"event_handler_fout_open: dup\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
    } else {
        tfd = open(
            opt_path_output,
            0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
            0o400 as libc::c_int | 0o200 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int,
        );
        if tfd < 0 as libc::c_int {
            perror(
                b"event_handler_fout_open: open\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
    }
    *fd = tfd;
    return 0 as libc::c_int;
}
