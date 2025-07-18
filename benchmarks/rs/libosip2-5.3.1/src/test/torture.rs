use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_strncasecmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn osip_message_init(sip: *mut *mut osip_message_t) -> libc::c_int;
    fn osip_message_free(sip: *mut osip_message_t);
    fn osip_message_parse(
        sip: *mut osip_message_t,
        buf: *const libc::c_char,
        length: size_t,
    ) -> libc::c_int;
    fn osip_message_to_str(
        sip: *mut osip_message_t,
        dest: *mut *mut libc::c_char,
        message_length: *mut size_t,
    ) -> libc::c_int;
    fn osip_message_clone(
        sip: *const osip_message_t,
        dest: *mut *mut osip_message_t,
    ) -> libc::c_int;
    fn parser_init() -> libc::c_int;
    fn osip_message_force_update(sip: *mut osip_message_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __node {
    pub next: *mut __node_t,
    pub element: *mut libc::c_void,
}
pub type __node_t = __node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_list {
    pub nb_elt: libc::c_int,
    pub node: *mut __node_t,
}
pub type osip_list_t = osip_list;
pub type osip_malloc_func_t = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type osip_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_uri {
    pub scheme: *mut libc::c_char,
    pub username: *mut libc::c_char,
    pub password: *mut libc::c_char,
    pub host: *mut libc::c_char,
    pub port: *mut libc::c_char,
    pub url_params: osip_list_t,
    pub url_headers: osip_list_t,
    pub string: *mut libc::c_char,
}
pub type osip_uri_t = osip_uri;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_content_type {
    pub type_0: *mut libc::c_char,
    pub subtype: *mut libc::c_char,
    pub gen_params: osip_list_t,
}
pub type osip_content_type_t = osip_content_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_content_length {
    pub value: *mut libc::c_char,
}
pub type osip_content_length_t = osip_content_length;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_call_id {
    pub number: *mut libc::c_char,
    pub host: *mut libc::c_char,
}
pub type osip_call_id_t = osip_call_id;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_from {
    pub displayname: *mut libc::c_char,
    pub url: *mut osip_uri_t,
    pub gen_params: osip_list_t,
}
pub type osip_from_t = osip_from;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_cseq {
    pub method: *mut libc::c_char,
    pub number: *mut libc::c_char,
}
pub type osip_cseq_t = osip_cseq;
pub type osip_mime_version_t = osip_content_length_t;
pub type osip_to_t = osip_from_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_message {
    pub sip_version: *mut libc::c_char,
    pub req_uri: *mut osip_uri_t,
    pub sip_method: *mut libc::c_char,
    pub status_code: libc::c_int,
    pub reason_phrase: *mut libc::c_char,
    pub accepts: osip_list_t,
    pub accept_encodings: osip_list_t,
    pub accept_languages: osip_list_t,
    pub alert_infos: osip_list_t,
    pub allows: osip_list_t,
    pub authentication_infos: osip_list_t,
    pub authorizations: osip_list_t,
    pub call_id: *mut osip_call_id_t,
    pub call_infos: osip_list_t,
    pub contacts: osip_list_t,
    pub content_encodings: osip_list_t,
    pub content_length: *mut osip_content_length_t,
    pub content_type: *mut osip_content_type_t,
    pub cseq: *mut osip_cseq_t,
    pub error_infos: osip_list_t,
    pub from: *mut osip_from_t,
    pub mime_version: *mut osip_mime_version_t,
    pub proxy_authenticates: osip_list_t,
    pub proxy_authentication_infos: osip_list_t,
    pub proxy_authorizations: osip_list_t,
    pub record_routes: osip_list_t,
    pub routes: osip_list_t,
    pub to: *mut osip_to_t,
    pub vias: osip_list_t,
    pub www_authenticates: osip_list_t,
    pub headers: osip_list_t,
    pub bodies: osip_list_t,
    pub message_property: libc::c_int,
    pub message: *mut libc::c_char,
    pub message_length: size_t,
    pub application_data: *mut libc::c_void,
}
pub type osip_message_t = osip_message;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
unsafe extern "C" fn usage() {
    fprintf(
        stderr,
        b"Usage: ./torture_test torture_file [-v (verbose)] [-c (clone)] [-p (performance: loop 100.000]\n\0"
            as *const u8 as *const libc::c_char,
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn read_binary(
    mut msg: *mut *mut libc::c_char,
    mut len: *mut libc::c_int,
    mut torture_file: *mut FILE,
) -> libc::c_int {
    *msg = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(100001 as libc::c_int as size_t)
    } else {
        malloc(100001 as libc::c_int as libc::c_ulong)
    }) as *mut libc::c_char;
    *len = fread(
        *msg as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        100000 as libc::c_int as libc::c_ulong,
        torture_file,
    ) as libc::c_int;
    return if ferror(torture_file) != 0 {
        -(1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut success: libc::c_int = 1 as libc::c_int;
    let mut loop_0: libc::c_int = 1 as libc::c_int;
    let mut verbose: libc::c_int = 0 as libc::c_int;
    let mut clone: libc::c_int = 0 as libc::c_int;
    let mut torture_file: *mut FILE = 0 as *mut FILE;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pos: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut expected_error: libc::c_int = 0 as libc::c_int;
    pos = 2 as libc::c_int;
    while pos < argc {
        if 0 as libc::c_int
            == strncmp(
                *argv.offset(pos as isize),
                b"-v\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            )
        {
            verbose = 1 as libc::c_int;
        } else if 0 as libc::c_int
            == strncmp(
                *argv.offset(pos as isize),
                b"-c\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            )
        {
            clone = 1 as libc::c_int;
        } else if 0 as libc::c_int
            == strncmp(
                *argv.offset(pos as isize),
                b"-p\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            )
        {
            loop_0 = 100000 as libc::c_int;
        } else {
            usage();
        }
        pos += 1;
        pos;
    }
    if argc < 2 as libc::c_int {
        usage();
    }
    torture_file = fopen(
        *argv.offset(1 as libc::c_int as isize),
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if torture_file.is_null() {
        usage();
    }
    parser_init();
    if read_binary(&mut msg, &mut len, torture_file) < 0 as libc::c_int {
        fprintf(
            stdout,
            b"test %s : ============================ FAILED (cannot read file)\n\0"
                as *const u8 as *const libc::c_char,
            *argv.offset(1 as libc::c_int as isize),
        );
        return -(999 as libc::c_int);
    }
    ptr = msg;
    if osip_strncasecmp(
        msg,
        b"expected_error:\0" as *const u8 as *const libc::c_char,
        strlen(b"expected_error:\0" as *const u8 as *const libc::c_char),
    ) == 0 as libc::c_int
    {
        ptr = ptr
            .offset(
                strlen(b"expected_error:\0" as *const u8 as *const libc::c_char) as isize,
            );
        len = (len as libc::c_ulong)
            .wrapping_sub(
                strlen(b"expected_error:\0" as *const u8 as *const libc::c_char),
            ) as libc::c_int as libc::c_int;
        expected_error = atoi(ptr);
        while *ptr as libc::c_int != '\r' as i32 && *ptr as libc::c_int != '\n' as i32 {
            ptr = ptr.offset(1);
            ptr;
            len -= 1;
            len;
        }
        while *ptr as libc::c_int == '\r' as i32 || *ptr as libc::c_int == '\n' as i32 {
            ptr = ptr.offset(1);
            ptr;
            len -= 1;
            len;
        }
    }
    success = test_message(ptr, len as size_t, verbose, clone, loop_0);
    if verbose != 0 {
        fprintf(
            stdout,
            b"test %s : ============================ \n\0" as *const u8
                as *const libc::c_char,
            *argv.offset(1 as libc::c_int as isize),
        );
        fwrite(
            msg as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            len as libc::c_ulong,
            stdout,
        );
    }
    if success == expected_error {
        fprintf(
            stdout,
            b"test %s : ============================ OK (error_code=%i)\n\0" as *const u8
                as *const libc::c_char,
            *argv.offset(1 as libc::c_int as isize),
            expected_error,
        );
    } else {
        fprintf(
            stdout,
            b"test %s : ============================ FAILED (expected=%i error_code=%i)\n\0"
                as *const u8 as *const libc::c_char,
            *argv.offset(1 as libc::c_int as isize),
            expected_error,
            success,
        );
    }
    if !msg.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(msg as *mut libc::c_void);
        } else {
            free(msg as *mut libc::c_void);
        }
    }
    fclose(torture_file);
    if success != expected_error {
        exit(1 as libc::c_int);
    } else {
        exit(0 as libc::c_int);
    };
}
pub unsafe extern "C" fn test_message(
    mut msg: *mut libc::c_char,
    mut len: size_t,
    mut verbose: libc::c_int,
    mut clone: libc::c_int,
    mut perf: libc::c_int,
) -> libc::c_int {
    let mut sip: *mut osip_message_t = 0 as *mut osip_message_t;
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: libc::c_int = perf;
    if verbose != 0 {
        fprintf(
            stdout,
            b"Trying %i sequentials calls to osip_message_init(), osip_message_parse() and osip_message_free()\n\0"
                as *const u8 as *const libc::c_char,
            j,
        );
    }
    while j != 0 as libc::c_int {
        j -= 1;
        j;
        osip_message_init(&mut sip);
        err = osip_message_parse(sip, msg, len);
        if err != 0 as libc::c_int {
            if verbose != 0 {
                fprintf(
                    stdout,
                    b"ERROR: failed while parsing!\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            osip_message_free(sip);
            return err;
        }
        osip_message_free(sip);
    }
    osip_message_init(&mut sip);
    err = osip_message_parse(sip, msg, len);
    if err != 0 as libc::c_int {
        if verbose != 0 {
            fprintf(
                stdout,
                b"ERROR: failed while parsing!\n\0" as *const u8 as *const libc::c_char,
            );
        }
        osip_message_free(sip);
        return err;
    } else {
        let mut length: size_t = 0;
        osip_message_force_update(sip);
        err = osip_message_to_str(sip, &mut result, &mut length);
        if err != 0 as libc::c_int {
            if verbose != 0 {
                fprintf(
                    stdout,
                    b"ERROR: failed while printing message!\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            osip_message_free(sip);
            return err;
        } else {
            if verbose != 0 {
                fwrite(
                    result as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    length,
                    stdout,
                );
            }
            if clone != 0 {
                let mut j_0: libc::c_int = perf;
                if verbose != 0 {
                    fprintf(
                        stdout,
                        b"Trying %i sequentials calls to osip_message_clone() and osip_message_free()\n\0"
                            as *const u8 as *const libc::c_char,
                        j_0,
                    );
                }
                while j_0 != 0 as libc::c_int {
                    let mut copy: *mut osip_message_t = 0 as *mut osip_message_t;
                    j_0 -= 1;
                    j_0;
                    err = osip_message_clone(sip, &mut copy);
                    if err != 0 as libc::c_int {
                        if verbose != 0 {
                            fprintf(
                                stdout,
                                b"ERROR: failed while creating copy of message!\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        break;
                    } else {
                        let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut length_0: size_t = 0;
                        osip_message_force_update(copy);
                        err = osip_message_to_str(copy, &mut tmp, &mut length_0);
                        if err != 0 as libc::c_int {
                            if verbose != 0 {
                                fprintf(
                                    stdout,
                                    b"ERROR: failed while printing message!\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                        } else {
                            if 0 as libc::c_int == strcmp(result, tmp) {
                                if verbose != 0 {
                                    printf(
                                        b"The osip_message_clone method works perfectly\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                }
                            } else {
                                printf(
                                    b"ERROR: The osip_message_clone method DOES NOT works\n\0"
                                        as *const u8 as *const libc::c_char,
                                );
                                err = -(1 as libc::c_int);
                                if verbose != 0 {
                                    printf(
                                        b"Here is the copy: \n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    fwrite(
                                        tmp as *const libc::c_void,
                                        1 as libc::c_int as libc::c_ulong,
                                        length_0,
                                        stdout,
                                    );
                                    printf(b"\n\0" as *const u8 as *const libc::c_char);
                                }
                            }
                            if !tmp.is_null() {
                                if osip_free_func.is_some() {
                                    osip_free_func.unwrap()(tmp as *mut libc::c_void);
                                } else {
                                    free(tmp as *mut libc::c_void);
                                }
                            }
                        }
                        osip_message_free(copy);
                    }
                }
                if verbose != 0 {
                    fprintf(
                        stdout,
                        b"sequentials calls: done\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            if !result.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(result as *mut libc::c_void);
                } else {
                    free(result as *mut libc::c_void);
                }
            }
        }
        osip_message_free(sip);
    }
    return err;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
