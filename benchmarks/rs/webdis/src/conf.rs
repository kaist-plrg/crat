use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn getuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
    fn __errno_location() -> *mut libc::c_int;
    fn json_delete(json: *mut json_t);
    fn json_object_get(object: *const json_t, key: *const libc::c_char) -> *mut json_t;
    fn json_object_iter(object: *mut json_t) -> *mut libc::c_void;
    fn json_object_iter_next(
        object: *mut json_t,
        iter: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn json_object_iter_key(iter: *mut libc::c_void) -> *const libc::c_char;
    fn json_object_iter_value(iter: *mut libc::c_void) -> *mut json_t;
    fn json_array_size(array: *const json_t) -> size_t;
    fn json_array_get(array: *const json_t, index: size_t) -> *mut json_t;
    fn json_string_value(string: *const json_t) -> *const libc::c_char;
    fn json_integer_value(integer: *const json_t) -> json_int_t;
    fn json_load_file(
        path: *const libc::c_char,
        flags: size_t,
        error: *mut json_error_t,
    ) -> *mut json_t;
    fn base64_init_encodestate(state_in: *mut base64_encodestate);
    fn base64_encode_block(
        plaintext_in: *const libc::c_char,
        length_in: libc::c_int,
        code_out: *mut libc::c_char,
        state_in: *mut base64_encodestate,
    ) -> libc::c_int;
    fn base64_encode_blockend(
        code_out: *mut libc::c_char,
        state_in: *mut base64_encodestate,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
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
pub type uint32_t = __uint32_t;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
pub type json_type = libc::c_uint;
pub const JSON_NULL: json_type = 7;
pub const JSON_FALSE: json_type = 6;
pub const JSON_TRUE: json_type = 5;
pub const JSON_REAL: json_type = 4;
pub const JSON_INTEGER: json_type = 3;
pub const JSON_STRING: json_type = 2;
pub const JSON_ARRAY: json_type = 1;
pub const JSON_OBJECT: json_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_t {
    pub type_0: json_type,
    pub refcount: size_t,
}
pub type json_int_t = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_error_t {
    pub text: [libc::c_char; 160],
    pub line: libc::c_int,
    pub column: libc::c_int,
    pub source: [libc::c_char; 80],
}
pub type base64_encodestep = libc::c_uint;
pub const step_C: base64_encodestep = 2;
pub const step_B: base64_encodestep = 1;
pub const step_A: base64_encodestep = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_encodestate {
    pub step: base64_encodestep,
    pub result: libc::c_char,
    pub stepcount: libc::c_int,
}
pub type log_level = libc::c_uint;
pub const WEBDIS_TRACE: log_level = 8;
pub const WEBDIS_DEBUG: log_level = 4;
pub const WEBDIS_INFO: log_level = 3;
pub const WEBDIS_NOTICE: log_level = 2;
pub const WEBDIS_WARNING: log_level = 1;
pub const WEBDIS_ERROR: log_level = 0;
pub type log_fsync_mode = libc::c_uint;
pub const LOG_FSYNC_ALL: log_fsync_mode = 2;
pub const LOG_FSYNC_MILLIS: log_fsync_mode = 1;
pub const LOG_FSYNC_AUTO: log_fsync_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct auth {
    pub use_legacy_auth: libc::c_int,
    pub username: *mut libc::c_char,
    pub password: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conf {
    pub redis_host: *mut libc::c_char,
    pub redis_port: libc::c_int,
    pub redis_auth: *mut auth,
    pub http_host: *mut libc::c_char,
    pub http_port: libc::c_int,
    pub http_threads: libc::c_int,
    pub http_max_request_size: size_t,
    pub pool_size_per_thread: libc::c_int,
    pub daemonize: libc::c_int,
    pub pidfile: *mut libc::c_char,
    pub websockets: libc::c_int,
    pub database: libc::c_int,
    pub perms: *mut acl,
    pub user: uid_t,
    pub group: gid_t,
    pub logfile: *mut libc::c_char,
    pub verbosity: log_level,
    pub log_fsync: C2RustUnnamed_0,
    pub hiredis_opts: C2RustUnnamed,
    pub default_root: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub keep_alive_sec: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub mode: log_fsync_mode,
    pub period_millis: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct acl {
    pub cidr: C2RustUnnamed_1,
    pub http_basic_auth: *mut libc::c_char,
    pub enabled: acl_commands,
    pub disabled: acl_commands,
    pub next: *mut acl,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct acl_commands {
    pub count: libc::c_uint,
    pub commands: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub enabled: libc::c_int,
    pub subnet: in_addr_t,
    pub mask: in_addr_t,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
#[inline]
unsafe extern "C" fn json_decref(mut json: *mut json_t) {
    if !json.is_null() && (*json).refcount != -(1 as libc::c_int) as size_t
        && {
            (*json).refcount = ((*json).refcount).wrapping_sub(1);
            (*json).refcount == 0 as libc::c_int as libc::c_ulong
        }
    {
        json_delete(json);
    }
}
pub unsafe extern "C" fn conf_str_allcaps(
    mut s: *const libc::c_char,
    sz: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < sz {
        if *s.offset(i as isize) as libc::c_int
            != ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *s.offset(i as isize) as libc::c_int;
                        __res = (if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = toupper(*s.offset(i as isize) as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(*s.offset(i as isize) as libc::c_int as isize);
                }
                __res
            })
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn conf_string_or_envvar(
    mut val: *const libc::c_char,
) -> *mut libc::c_char {
    if val.is_null() {
        return strdup(b"\0" as *const u8 as *const libc::c_char);
    }
    let mut val_len: size_t = strlen(val);
    if val_len >= 2 as libc::c_int as libc::c_ulong
        && *val.offset(0 as libc::c_int as isize) as libc::c_int == '$' as i32
        && conf_str_allcaps(
            val.offset(1 as libc::c_int as isize),
            val_len.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) != 0
    {
        let mut env_val: *mut libc::c_char = getenv(
            val.offset(1 as libc::c_int as isize),
        );
        if !env_val.is_null() {
            return strdup(env_val)
        } else {
            fprintf(
                stderr,
                b"No value found for env var %s\n\0" as *const u8 as *const libc::c_char,
                val.offset(1 as libc::c_int as isize),
            );
        }
    }
    return strdup(val);
}
pub unsafe extern "C" fn atoi_free(mut s: *mut libc::c_char) -> libc::c_int {
    let mut val: libc::c_int = atoi(s);
    free(s as *mut libc::c_void);
    return val;
}
pub unsafe extern "C" fn is_true_free(mut s: *mut libc::c_char) -> libc::c_int {
    let mut val: libc::c_int = if strcasecmp(
        s,
        b"true\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    free(s as *mut libc::c_void);
    return val;
}
pub unsafe extern "C" fn conf_read(mut filename: *const libc::c_char) -> *mut conf {
    let mut j: *mut json_t = 0 as *mut json_t;
    let mut error: json_error_t = json_error_t {
        text: [0; 160],
        line: 0,
        column: 0,
        source: [0; 80],
    };
    let mut conf: *mut conf = 0 as *mut conf;
    let mut kv: *mut libc::c_void = 0 as *mut libc::c_void;
    conf = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<conf>() as libc::c_ulong,
    ) as *mut conf;
    (*conf).redis_host = strdup(b"127.0.0.1\0" as *const u8 as *const libc::c_char);
    (*conf).redis_port = 6379 as libc::c_int;
    (*conf).http_host = strdup(b"0.0.0.0\0" as *const u8 as *const libc::c_char);
    (*conf).http_port = 7379 as libc::c_int;
    (*conf)
        .http_max_request_size = (128 as libc::c_int * 1024 as libc::c_int
        * 1024 as libc::c_int) as size_t;
    (*conf).http_threads = 4 as libc::c_int;
    (*conf).user = getuid();
    (*conf).group = getgid();
    (*conf)
        .logfile = b"webdis.log\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    (*conf).log_fsync.mode = LOG_FSYNC_AUTO;
    (*conf).verbosity = WEBDIS_NOTICE;
    (*conf).daemonize = 0 as libc::c_int;
    (*conf)
        .pidfile = b"webdis.pid\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    (*conf).database = 0 as libc::c_int;
    (*conf).pool_size_per_thread = 2 as libc::c_int;
    j = json_load_file(filename, 0 as libc::c_int as size_t, &mut error);
    if j.is_null() {
        fprintf(
            stderr,
            b"Error: %s (line %d)\n\0" as *const u8 as *const libc::c_char,
            (error.text).as_mut_ptr(),
            error.line,
        );
        return conf;
    }
    kv = json_object_iter(j);
    while !kv.is_null() {
        let mut jtmp: *mut json_t = json_object_iter_value(kv);
        if strcmp(
            json_object_iter_key(kv),
            b"redis_host\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_STRING as libc::c_int as libc::c_uint
        {
            free((*conf).redis_host as *mut libc::c_void);
            (*conf).redis_host = conf_string_or_envvar(json_string_value(jtmp));
        } else if strcmp(
            json_object_iter_key(kv),
            b"redis_port\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_INTEGER as libc::c_int as libc::c_uint
        {
            (*conf).redis_port = json_integer_value(jtmp) as libc::c_int;
        } else if strcmp(
            json_object_iter_key(kv),
            b"redis_port\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_STRING as libc::c_int as libc::c_uint
        {
            (*conf)
                .redis_port = atoi_free(conf_string_or_envvar(json_string_value(jtmp)));
        } else if strcmp(
            json_object_iter_key(kv),
            b"redis_auth\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if (*jtmp).type_0 as libc::c_uint
                == JSON_STRING as libc::c_int as libc::c_uint
            {
                (*conf)
                    .redis_auth = conf_auth_legacy(
                    conf_string_or_envvar(json_string_value(jtmp)),
                );
            } else if (*jtmp).type_0 as libc::c_uint
                == JSON_ARRAY as libc::c_int as libc::c_uint
            {
                (*conf).redis_auth = conf_auth_username_password(jtmp);
            } else if (*jtmp).type_0 as libc::c_uint
                != JSON_NULL as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"Config error with 'redis_auth': expected a string or an array of two strings. Starting with auth disabled.\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        } else if strcmp(
            json_object_iter_key(kv),
            b"http_host\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_STRING as libc::c_int as libc::c_uint
        {
            free((*conf).http_host as *mut libc::c_void);
            (*conf).http_host = conf_string_or_envvar(json_string_value(jtmp));
        } else if strcmp(
            json_object_iter_key(kv),
            b"http_port\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_INTEGER as libc::c_int as libc::c_uint
        {
            (*conf).http_port = json_integer_value(jtmp) as libc::c_int;
        } else if strcmp(
            json_object_iter_key(kv),
            b"http_port\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_STRING as libc::c_int as libc::c_uint
        {
            (*conf)
                .http_port = atoi_free(conf_string_or_envvar(json_string_value(jtmp)));
        } else if strcmp(
            json_object_iter_key(kv),
            b"http_max_request_size\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_INTEGER as libc::c_int as libc::c_uint
        {
            (*conf).http_max_request_size = json_integer_value(jtmp) as size_t;
        } else if strcmp(
            json_object_iter_key(kv),
            b"http_max_request_size\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_STRING as libc::c_int as libc::c_uint
        {
            (*conf)
                .http_max_request_size = atoi_free(
                conf_string_or_envvar(json_string_value(jtmp)),
            ) as size_t;
        } else if strcmp(
            json_object_iter_key(kv),
            b"threads\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_INTEGER as libc::c_int as libc::c_uint
        {
            (*conf).http_threads = json_integer_value(jtmp) as libc::c_int;
        } else if strcmp(
            json_object_iter_key(kv),
            b"threads\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_STRING as libc::c_int as libc::c_uint
        {
            (*conf)
                .http_threads = atoi_free(
                conf_string_or_envvar(json_string_value(jtmp)),
            );
        } else if strcmp(
            json_object_iter_key(kv),
            b"acl\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_ARRAY as libc::c_int as libc::c_uint
        {
            (*conf).perms = conf_parse_acls(jtmp);
        } else if strcmp(
            json_object_iter_key(kv),
            b"user\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_STRING as libc::c_int as libc::c_uint
        {
            let mut u: *mut passwd = 0 as *mut passwd;
            let mut username: *mut libc::c_char = conf_string_or_envvar(
                json_string_value(jtmp),
            );
            *__errno_location() = 0 as libc::c_int;
            u = getpwnam(username);
            if !u.is_null() {
                (*conf).user = (*u).pw_uid;
            } else if *__errno_location() != 0 {
                fprintf(
                    stderr,
                    b"Could not find user ID for user '%s', an error occurred: %s\n\0"
                        as *const u8 as *const libc::c_char,
                    username,
                    strerror(*__errno_location()),
                );
            } else {
                fprintf(
                    stderr,
                    b"Could not find user ID for unknown user '%s'\n\0" as *const u8
                        as *const libc::c_char,
                    username,
                );
            }
            free(username as *mut libc::c_void);
        } else if strcmp(
            json_object_iter_key(kv),
            b"group\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_STRING as libc::c_int as libc::c_uint
        {
            let mut g: *mut group = 0 as *mut group;
            let mut groupname: *mut libc::c_char = conf_string_or_envvar(
                json_string_value(jtmp),
            );
            *__errno_location() = 0 as libc::c_int;
            g = getgrnam(groupname);
            if !g.is_null() {
                (*conf).group = (*g).gr_gid;
            } else if *__errno_location() != 0 {
                fprintf(
                    stderr,
                    b"Could not find group ID for group '%s', an error occurred: %s\n\0"
                        as *const u8 as *const libc::c_char,
                    groupname,
                    strerror(*__errno_location()),
                );
            } else {
                fprintf(
                    stderr,
                    b"Could not find group ID for unknown group '%s'\n\0" as *const u8
                        as *const libc::c_char,
                    groupname,
                );
            }
            free(groupname as *mut libc::c_void);
        } else if strcmp(
            json_object_iter_key(kv),
            b"logfile\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_STRING as libc::c_int as libc::c_uint
        {
            (*conf).logfile = conf_string_or_envvar(json_string_value(jtmp));
        } else if strcmp(
            json_object_iter_key(kv),
            b"log_fsync\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if (*jtmp).type_0 as libc::c_uint
                == JSON_STRING as libc::c_int as libc::c_uint
                && strcmp(
                    json_string_value(jtmp),
                    b"auto\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                (*conf).log_fsync.mode = LOG_FSYNC_AUTO;
            } else if (*jtmp).type_0 as libc::c_uint
                == JSON_STRING as libc::c_int as libc::c_uint
                && strcmp(
                    json_string_value(jtmp),
                    b"all\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                (*conf).log_fsync.mode = LOG_FSYNC_ALL;
            } else if (*jtmp).type_0 as libc::c_uint
                == JSON_INTEGER as libc::c_int as libc::c_uint
                && json_integer_value(jtmp) > 0 as libc::c_int as libc::c_longlong
            {
                (*conf).log_fsync.mode = LOG_FSYNC_MILLIS;
                (*conf)
                    .log_fsync
                    .period_millis = json_integer_value(jtmp) as libc::c_int;
            } else if (*jtmp).type_0 as libc::c_uint
                != JSON_NULL as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"Unexpected value for \"log_fsync\", defaulting to \"auto\"\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        } else if strcmp(
            json_object_iter_key(kv),
            b"verbosity\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_INTEGER as libc::c_int as libc::c_uint
        {
            let mut tmp: libc::c_int = json_integer_value(jtmp) as libc::c_int;
            if tmp < 0 as libc::c_int || tmp > WEBDIS_TRACE as libc::c_int {
                fprintf(
                    stderr,
                    b"Invalid log verbosity: %d. Acceptable range: [%d .. %d]\n\0"
                        as *const u8 as *const libc::c_char,
                    tmp,
                    WEBDIS_ERROR as libc::c_int,
                    WEBDIS_TRACE as libc::c_int,
                );
            }
            (*conf)
                .verbosity = (if tmp < 0 as libc::c_int {
                WEBDIS_ERROR as libc::c_int as libc::c_uint
            } else if tmp > WEBDIS_TRACE as libc::c_int {
                WEBDIS_TRACE as libc::c_int as libc::c_uint
            } else {
                tmp as log_level as libc::c_uint
            }) as log_level;
        } else if strcmp(
            json_object_iter_key(kv),
            b"daemonize\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && ((*jtmp).type_0 as libc::c_uint
                == JSON_TRUE as libc::c_int as libc::c_uint
                || (*jtmp).type_0 as libc::c_uint
                    == JSON_FALSE as libc::c_int as libc::c_uint)
        {
            (*conf)
                .daemonize = if (*jtmp).type_0 as libc::c_uint
                == JSON_TRUE as libc::c_int as libc::c_uint
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            };
        } else if strcmp(
            json_object_iter_key(kv),
            b"daemonize\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_STRING as libc::c_int as libc::c_uint
        {
            (*conf)
                .daemonize = is_true_free(
                conf_string_or_envvar(json_string_value(jtmp)),
            );
        } else if strcmp(
            json_object_iter_key(kv),
            b"pidfile\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_STRING as libc::c_int as libc::c_uint
        {
            (*conf).pidfile = conf_string_or_envvar(json_string_value(jtmp));
        } else if strcmp(
            json_object_iter_key(kv),
            b"websockets\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && ((*jtmp).type_0 as libc::c_uint
                == JSON_TRUE as libc::c_int as libc::c_uint
                || (*jtmp).type_0 as libc::c_uint
                    == JSON_FALSE as libc::c_int as libc::c_uint)
        {
            (*conf)
                .websockets = if (*jtmp).type_0 as libc::c_uint
                == JSON_TRUE as libc::c_int as libc::c_uint
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            };
        } else if strcmp(
            json_object_iter_key(kv),
            b"websockets\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_STRING as libc::c_int as libc::c_uint
        {
            (*conf)
                .websockets = is_true_free(
                conf_string_or_envvar(json_string_value(jtmp)),
            );
        } else if strcmp(
            json_object_iter_key(kv),
            b"database\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_INTEGER as libc::c_int as libc::c_uint
        {
            (*conf).database = json_integer_value(jtmp) as libc::c_int;
        } else if strcmp(
            json_object_iter_key(kv),
            b"database\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_STRING as libc::c_int as libc::c_uint
        {
            (*conf).database = atoi_free(conf_string_or_envvar(json_string_value(jtmp)));
        } else if strcmp(
            json_object_iter_key(kv),
            b"pool_size\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_INTEGER as libc::c_int as libc::c_uint
        {
            (*conf).pool_size_per_thread = json_integer_value(jtmp) as libc::c_int;
        } else if strcmp(
            json_object_iter_key(kv),
            b"pool_size\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_STRING as libc::c_int as libc::c_uint
        {
            (*conf)
                .pool_size_per_thread = atoi_free(
                conf_string_or_envvar(json_string_value(jtmp)),
            );
        } else if strcmp(
            json_object_iter_key(kv),
            b"default_root\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_STRING as libc::c_int as libc::c_uint
        {
            (*conf).default_root = conf_string_or_envvar(json_string_value(jtmp));
        } else if strcmp(
            json_object_iter_key(kv),
            b"hiredis\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_OBJECT as libc::c_int as libc::c_uint
        {
            conf_parse_hiredis(conf, jtmp);
        } else {
            fprintf(
                stderr,
                b"Warning! Unexpected key or incorrect value in %s: '%s'\n\0"
                    as *const u8 as *const libc::c_char,
                filename,
                json_object_iter_key(kv),
            );
        }
        kv = json_object_iter_next(j, kv);
    }
    json_decref(j);
    return conf;
}
pub unsafe extern "C" fn acl_read_commands(
    mut jlist: *mut json_t,
    mut ac: *mut acl_commands,
) {
    let mut i: libc::c_uint = 0;
    let mut n: libc::c_uint = 0;
    let mut cur: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    n = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < json_array_size(jlist) {
        let mut jelem: *mut json_t = json_array_get(jlist, i as size_t);
        if (*jelem).type_0 as libc::c_uint == JSON_STRING as libc::c_int as libc::c_uint
        {
            n = n.wrapping_add(1);
            n;
        }
        i = i.wrapping_add(1);
        i;
    }
    (*ac)
        .commands = calloc(
        n as size_t,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    (*ac).count = n;
    i = 0 as libc::c_int as libc::c_uint;
    cur = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < json_array_size(jlist) {
        let mut jelem_0: *mut json_t = json_array_get(jlist, i as size_t);
        if (*jelem_0).type_0 as libc::c_uint
            == JSON_STRING as libc::c_int as libc::c_uint
        {
            let ref mut fresh0 = *((*ac).commands).offset(cur as isize);
            *fresh0 = conf_string_or_envvar(json_string_value(jelem_0));
            cur = cur.wrapping_add(1);
            cur;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn conf_parse_acl(mut j: *mut json_t) -> *mut acl {
    let mut jcidr: *mut json_t = 0 as *mut json_t;
    let mut jbasic: *mut json_t = 0 as *mut json_t;
    let mut jlist: *mut json_t = 0 as *mut json_t;
    let mut mask_bits: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    let mut a: *mut acl = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<acl>() as libc::c_ulong,
    ) as *mut acl;
    jcidr = json_object_get(j, b"ip\0" as *const u8 as *const libc::c_char);
    if !jcidr.is_null()
        && (*jcidr).type_0 as libc::c_uint == JSON_STRING as libc::c_int as libc::c_uint
    {
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
        s = conf_string_or_envvar(json_string_value(jcidr));
        p = strchr(s, '/' as i32);
        if p.is_null() {
            ip = strdup(s);
        } else {
            ip = calloc(
                (p.offset_from(s) as libc::c_long + 1 as libc::c_int as libc::c_long)
                    as size_t,
                1 as libc::c_int as libc::c_ulong,
            ) as *mut libc::c_char;
            memcpy(
                ip as *mut libc::c_void,
                s as *const libc::c_void,
                p.offset_from(s) as libc::c_long as size_t,
            );
            mask_bits = atoi(p.offset(1 as libc::c_int as isize)) as libc::c_ushort;
        }
        (*a).cidr.enabled = 1 as libc::c_int;
        (*a)
            .cidr
            .mask = if mask_bits as libc::c_int == 0 as libc::c_int {
            0xffffffff as libc::c_uint
        } else {
            (0xffffffff as libc::c_uint) << 32 as libc::c_int - mask_bits as libc::c_int
        };
        (*a).cidr.subnet = __bswap_32(inet_addr(ip)) & (*a).cidr.mask;
        free(ip as *mut libc::c_void);
    }
    jbasic = json_object_get(
        j,
        b"http_basic_auth\0" as *const u8 as *const libc::c_char,
    );
    if !jbasic.is_null()
        && (*jbasic).type_0 as libc::c_uint == JSON_STRING as libc::c_int as libc::c_uint
    {
        let mut b64: base64_encodestate = base64_encodestate {
            step: step_A,
            result: 0,
            stepcount: 0,
        };
        let mut pos: libc::c_int = 0;
        let mut p_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut plain: *mut libc::c_char = conf_string_or_envvar(
            json_string_value(jbasic),
        );
        let mut len: size_t = 0;
        let mut plain_len: size_t = (strlen(plain))
            .wrapping_add(0 as libc::c_int as libc::c_ulong);
        len = plain_len
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_div(6 as libc::c_int as libc::c_ulong);
        (*a)
            .http_basic_auth = calloc(len, 1 as libc::c_int as libc::c_ulong)
            as *mut libc::c_char;
        base64_init_encodestate(&mut b64);
        pos = base64_encode_block(
            plain,
            plain_len as libc::c_int,
            (*a).http_basic_auth,
            &mut b64,
        );
        free(plain as *mut libc::c_void);
        if pos == 0 {
            fprintf(
                stderr,
                b"Error: could not encode credentials as HTTP basic auth header\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        base64_encode_blockend(((*a).http_basic_auth).offset(pos as isize), &mut b64);
        p_0 = strchr(((*a).http_basic_auth).offset(pos as isize), '\n' as i32);
        if !p_0.is_null() {
            *p_0 = 0 as libc::c_int as libc::c_char;
        }
    }
    jlist = json_object_get(j, b"enabled\0" as *const u8 as *const libc::c_char);
    if !jlist.is_null()
        && (*jlist).type_0 as libc::c_uint == JSON_ARRAY as libc::c_int as libc::c_uint
    {
        acl_read_commands(jlist, &mut (*a).enabled);
    }
    jlist = json_object_get(j, b"disabled\0" as *const u8 as *const libc::c_char);
    if !jlist.is_null()
        && (*jlist).type_0 as libc::c_uint == JSON_ARRAY as libc::c_int as libc::c_uint
    {
        acl_read_commands(jlist, &mut (*a).disabled);
    }
    return a;
}
unsafe extern "C" fn conf_parse_acls(mut jtab: *mut json_t) -> *mut acl {
    let mut head: *mut acl = 0 as *mut acl;
    let mut tail: *mut acl = 0 as *mut acl;
    let mut tmp: *mut acl = 0 as *mut acl;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < json_array_size(jtab) {
        let mut val: *mut json_t = json_array_get(jtab, i as size_t);
        tmp = conf_parse_acl(val);
        if head.is_null() && tail.is_null() {
            tail = tmp;
            head = tail;
        } else {
            (*tail).next = tmp;
            tail = tmp;
        }
        i = i.wrapping_add(1);
        i;
    }
    return head;
}
pub unsafe extern "C" fn conf_free(mut conf: *mut conf) {
    free((*conf).redis_host as *mut libc::c_void);
    if !((*conf).redis_auth).is_null() {
        free((*(*conf).redis_auth).username as *mut libc::c_void);
        free((*(*conf).redis_auth).password as *mut libc::c_void);
    }
    free((*conf).redis_auth as *mut libc::c_void);
    free((*conf).http_host as *mut libc::c_void);
    free(conf as *mut libc::c_void);
}
unsafe extern "C" fn conf_auth_legacy(mut password: *mut libc::c_char) -> *mut auth {
    let mut ret: *mut auth = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<auth>() as libc::c_ulong,
    ) as *mut auth;
    if ret.is_null() {
        free(password as *mut libc::c_void);
        fprintf(
            stderr,
            b"Config error with 'redis_auth': failed to allocate memory for credentials. Starting with auth disabled.\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as *mut auth;
    }
    (*ret).use_legacy_auth = 1 as libc::c_int;
    (*ret).password = password;
    return ret;
}
unsafe extern "C" fn conf_auth_username_password(mut jarray: *mut json_t) -> *mut auth {
    let mut array_size: size_t = json_array_size(jarray);
    if array_size != 2 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"Config error with 'redis_auth': expected two values, found %zu. Starting with auth disabled.\n\0"
                as *const u8 as *const libc::c_char,
            array_size,
        );
        return 0 as *mut auth;
    }
    let mut jusername: *mut json_t = json_array_get(jarray, 0 as libc::c_int as size_t);
    let mut jpassword: *mut json_t = json_array_get(jarray, 1 as libc::c_int as size_t);
    if (*jusername).type_0 as libc::c_uint != JSON_STRING as libc::c_int as libc::c_uint
        || (*jpassword).type_0 as libc::c_uint
            != JSON_STRING as libc::c_int as libc::c_uint
    {
        fprintf(
            stderr,
            b"Config error with 'redis_auth': both values need to be strings. Starting with auth disabled.\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as *mut auth;
    }
    let mut username: *mut libc::c_char = conf_string_or_envvar(
        json_string_value(jusername),
    );
    let mut password: *mut libc::c_char = conf_string_or_envvar(
        json_string_value(jpassword),
    );
    let mut ret: *mut auth = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<auth>() as libc::c_ulong,
    ) as *mut auth;
    if username.is_null() || password.is_null() || ret.is_null() {
        free(username as *mut libc::c_void);
        free(password as *mut libc::c_void);
        free(ret as *mut libc::c_void);
        fprintf(
            stderr,
            b"Config error with 'redis_auth': failed to allocate memory for credentials. Starting with auth disabled.\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as *mut auth;
    }
    (*ret).use_legacy_auth = 0 as libc::c_int;
    (*ret).username = username;
    (*ret).password = password;
    return ret;
}
unsafe extern "C" fn conf_parse_hiredis(mut conf: *mut conf, mut jhiredis: *mut json_t) {
    let mut kv: *mut libc::c_void = json_object_iter(jhiredis);
    while !kv.is_null() {
        let mut jtmp: *mut json_t = json_object_iter_value(kv);
        let mut key: *const libc::c_char = json_object_iter_key(kv);
        if strcmp(key, b"keep_alive_sec\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            && (*jtmp).type_0 as libc::c_uint
                == JSON_INTEGER as libc::c_int as libc::c_uint
        {
            (*conf)
                .hiredis_opts
                .keep_alive_sec = json_integer_value(jtmp) as libc::c_int;
        } else {
            fprintf(
                stderr,
                b"Config error under 'hiredis': unknown key '%s'.\0" as *const u8
                    as *const libc::c_char,
                key,
            );
        }
        kv = json_object_iter_next(jhiredis, kv);
    }
}
