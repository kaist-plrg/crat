use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type URL_T;
    pub type AUTH_T;
    pub type ARRAY_T;
    pub type COOKIES_T;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
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
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn url_set_conttype(this: URL, type_0: *mut libc::c_char);
    fn url_set_postdata(this: URL, postdata: *mut libc::c_char, postlen: size_t);
    fn url_get_file(this: URL) -> *mut libc::c_char;
    static mut my: CONFIG;
    fn NOTIFY(L: LEVEL, fmt: *const libc::c_char, _: ...);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn trim(str: *mut libc::c_char) -> *mut libc::c_char;
    fn empty(s: *const libc::c_char) -> BOOLEAN;
    fn strmatch(str1: *mut libc::c_char, str2: *mut libc::c_char) -> BOOLEAN;
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
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type BOOLEAN = libc::c_uint;
pub const boolean_true: BOOLEAN = 1;
pub const boolean_false: BOOLEAN = 0;
pub type URL = *mut URL_T;
pub type METHOD = libc::c_uint;
pub const PATCH: METHOD = 9;
pub const CONNECT: METHOD = 8;
pub const OPTIONS: METHOD = 7;
pub const TRACE: METHOD = 6;
pub const DELETE: METHOD = 5;
pub const PUT: METHOD = 4;
pub const POST: METHOD = 3;
pub const GET: METHOD = 2;
pub const HEAD: METHOD = 1;
pub const NOMETHOD: METHOD = 0;
pub type AUTH = *mut AUTH_T;
pub type ARRAY = *mut ARRAY_T;
pub type COOKIES = *mut COOKIES_T;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LINES {
    pub index: libc::c_int,
    pub line: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONFIG {
    pub logging: BOOLEAN,
    pub shlog: BOOLEAN,
    pub limit: libc::c_int,
    pub url: *mut libc::c_char,
    pub logfile: [libc::c_char; 128],
    pub verbose: BOOLEAN,
    pub quiet: BOOLEAN,
    pub parser: BOOLEAN,
    pub csv: BOOLEAN,
    pub fullurl: BOOLEAN,
    pub display: BOOLEAN,
    pub config: BOOLEAN,
    pub color: BOOLEAN,
    pub cusers: libc::c_int,
    pub delay: libc::c_float,
    pub timeout: libc::c_int,
    pub bench: BOOLEAN,
    pub internet: BOOLEAN,
    pub timestamp: BOOLEAN,
    pub time: libc::c_int,
    pub secs: libc::c_int,
    pub reps: libc::c_int,
    pub file: [libc::c_char; 128],
    pub length: libc::c_int,
    pub nomap: *mut LINES,
    pub debug: BOOLEAN,
    pub chunked: BOOLEAN,
    pub unique: BOOLEAN,
    pub get: BOOLEAN,
    pub print: BOOLEAN,
    pub mark: BOOLEAN,
    pub markstr: *mut libc::c_char,
    pub protocol: libc::c_int,
    pub cookies: COOKIES,
    pub uagent: [libc::c_char; 256],
    pub encoding: [libc::c_char; 256],
    pub conttype: [libc::c_char; 256],
    pub bids: libc::c_int,
    pub auth: AUTH,
    pub keepalive: BOOLEAN,
    pub signaled: libc::c_int,
    pub extra: [libc::c_char; 2048],
    pub login: BOOLEAN,
    pub loginurl: *mut libc::c_char,
    pub lurl: ARRAY,
    pub failures: libc::c_int,
    pub failed: libc::c_int,
    pub escape: BOOLEAN,
    pub expire: BOOLEAN,
    pub follow: BOOLEAN,
    pub zero_ok: BOOLEAN,
    pub spinner: BOOLEAN,
    pub cache: BOOLEAN,
    pub rc: [libc::c_char; 256],
    pub ssl_timeout: libc::c_int,
    pub ssl_cert: *mut libc::c_char,
    pub ssl_key: *mut libc::c_char,
    pub ssl_ciphers: *mut libc::c_char,
    pub method: METHOD,
    pub json_output: BOOLEAN,
    pub cond: pthread_cond_t,
    pub lock: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ContentType {
    pub ext: *mut libc::c_char,
    pub ascii: BOOLEAN,
    pub type_0: *mut libc::c_char,
}
pub type LEVEL = libc::c_uint;
pub const FATAL: LEVEL = 3;
pub const ERROR: LEVEL = 2;
pub const WARNING: LEVEL = 1;
pub const DEBUG: LEVEL = 0;
static mut tmap: [ContentType; 168] = [
    {
        let mut init = ContentType {
            ext: b"default\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"application/x-www-form-urlencoded\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"ai\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/postscript\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"aif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"audio/x-aiff\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"aifc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"audio/x-aiff\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"aiff\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"audio/x-aiff\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"asc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"text/plain\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"au\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"audio/basic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"avi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"video/x-msvideo\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"bcpio\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-bcpio\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"bin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/octet-stream\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"text/plain\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"cc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"text/plain\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"ccad\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/clariscad\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"cdf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-netcdf\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"class\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/octet-stream\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"cpio\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-cpio\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"cpt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/mac-compactpro\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"csh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-csh\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"css\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"text/css\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"csv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"text/csv\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"dcr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-director\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"dir\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-director\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"dms\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/octet-stream\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"doc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/msword\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"drw\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/drafting\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"dvi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-dvi\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"dwg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/acad\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"dxf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/dxf\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"dxr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-director\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"eps\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/postscript\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"etx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"text/x-setext\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"exe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/octet-stream\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"ez\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/andrew-inset\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"f\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"text/plain\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"f90\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"text/plain\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"fli\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"video/x-fli\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"gif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"image/gif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"gtar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-gtar\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"gz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-gzip\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"h\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"text/plain\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"hdf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-hdf\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"hh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"text/plain\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"hqx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/mac-binhex40\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"htm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"text/html\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"html\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"text/html\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"ice\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"x-conference/x-cooltalk\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"ico\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"image/x-icon\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"ief\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"image/ief\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"iges\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"model/iges\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"igs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"model/iges\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"ips\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-ipscript\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"ipx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-ipix\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"jpe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"image/jpeg\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"jpeg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"image/jpeg\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"jpg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"image/jpeg\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"js\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-javascript\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"json\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/json\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"kar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"audio/midi\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"latex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-latex\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"lha\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/octet-stream\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"lsp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-lisp\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"lzh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/octet-stream\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"m\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"text/plain\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"man\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-troff-man\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"md\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"text/x-markdown\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"me\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-troff-me\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"mesh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"model/mesh\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"mid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"audio/midi\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"midi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"audio/midi\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"mif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/vnd.mif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"mime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"www/mime\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"mov\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"video/quicktime\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"movie\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"video/x-sgi-movie\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"mp2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"audio/mpeg\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"mp3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"audio/mpeg\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"mpe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"video/mpeg\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"mpeg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"video/mpeg\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"mpg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"video/mpeg\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"mpga\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"audio/mpeg\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"ms\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-troff-ms\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"msh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"model/mesh\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"nc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-netcdf\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"oda\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/oda\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"pbm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"image/x-portable-bitmap\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"pdb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"chemical/x-pdb\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"pdf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/pdf\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"pgm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"image/x-portable-graymap\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"pgn\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-chess-pgn\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"png\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"image/png\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"pnm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"image/x-portable-anymap\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"pot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/mspowerpoint\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"ppm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"image/x-portable-pixmap\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"pps\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/mspowerpoint\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"ppt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/mspowerpoint\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"ppz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/mspowerpoint\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"pre\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-freelance\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"proto\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-protobuf\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"prt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/pro_eng\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"ps\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/postscript\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"qt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"video/quicktime\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"ra\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"audio/x-realaudio\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"ram\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"audio/x-pn-realaudio\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"ras\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"image/cmu-raster\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"rgb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"image/x-rgb\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"rm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"audio/x-pn-realaudio\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"roff\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-troff\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"rpm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"audio/x-pn-realaudio-plugin\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"rtf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"text/rtf\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"rtx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"text/richtext\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"scm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-lotusscreencam\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"set\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/set\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"sgm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"text/sgml\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"sgml\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"text/sgml\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"sh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-sh\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"shar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-shar\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"silo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"model/mesh\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"sit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-stuffit\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"skd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-koan\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"skm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-koan\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"skp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-koan\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"skt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-koan\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"smi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/smil\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"smil\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/smil\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"snd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"audio/basic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"sol\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/solids\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"spl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-futuresplash\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"src\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-wais-source\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"step\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/STEP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"stl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/SLA\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"stp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/STEP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"sv4cpio\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-sv4cpio\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"sv4crc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-sv4crc\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"svg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"image/svg+xml\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"swf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-shockwave-flash\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"t\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-troff\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"tar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-tar\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"tcl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-tcl\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"tex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-tex\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"texi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-texinfo\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"texinfo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-texinfo\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"tif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"image/tiff\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"tiff\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"image/tiff\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"tr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-troff\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"tsi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"audio/TSP-audio\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"tsp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/dsptype\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"tsv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"text/tab-separated-values\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"txt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"text/plain\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"unv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/i-deas\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"ustar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-ustar\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"vcd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/x-cdlink\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"vda\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/vda\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"viv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"video/vnd.vivo\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"vivo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"video/vnd.vivo\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"vrml\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"model/vrml\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"audio/x-wav\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"wrl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"model/vrml\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"xbm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"image/x-xbitmap\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"xlc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/vnd.ms-excel\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"xll\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/vnd.ms-excel\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"xlm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/vnd.ms-excel\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"xls\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/vnd.ms-excel\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"xlw\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/vnd.ms-excel\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"xml\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"text/xml\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"xpm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"image/x-xpixmap\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"xwd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"image/x-xwindowdump\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"xyz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"chemical/x-pdb\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"yml\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_true,
            type_0: b"application/x-yaml\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = ContentType {
            ext: b"zip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ascii: boolean_false,
            type_0: b"application/zip\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
];
pub unsafe extern "C" fn get_file_extension(
    mut file: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut dot: *mut libc::c_char = strrchr(file, '.' as i32);
    if dot.is_null() || dot == file {
        return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    return dot.offset(1 as libc::c_int as isize);
}
pub unsafe extern "C" fn get_content_type(
    mut file: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut ext: *mut libc::c_char = 0 as *mut libc::c_char;
    ext = get_file_extension(file);
    i = 0 as libc::c_int;
    while i
        < ::std::mem::size_of::<[ContentType; 168]>() as libc::c_ulong as libc::c_int
            / ::std::mem::size_of::<ContentType>() as libc::c_ulong as libc::c_int
    {
        if strmatch(ext, tmap[i as usize].ext) as u64 != 0 {
            return tmap[i as usize].type_0;
        }
        i += 1;
        i;
    }
    return tmap[0 as libc::c_int as usize].type_0;
}
pub unsafe extern "C" fn is_ascii(mut file: *mut libc::c_char) -> BOOLEAN {
    let mut i: libc::c_int = 0;
    let mut ext: *mut libc::c_char = 0 as *mut libc::c_char;
    ext = get_file_extension(file);
    i = 0 as libc::c_int;
    while i
        < ::std::mem::size_of::<[ContentType; 168]>() as libc::c_ulong as libc::c_int
            / ::std::mem::size_of::<ContentType>() as libc::c_ulong as libc::c_int
    {
        if strmatch(ext, tmap[i as usize].ext) as u64 != 0 {
            return tmap[i as usize].ascii;
        }
        i += 1;
        i;
    }
    return tmap[0 as libc::c_int as usize].ascii;
}
pub unsafe extern "C" fn load_file(mut U: URL, mut file: *mut libc::c_char) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut len: size_t = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mode: [libc::c_char; 8] = [0; 8];
    filename = trim(file);
    memset(
        mode.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
    );
    snprintf(
        mode.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        if is_ascii(filename) as libc::c_uint != 0 {
            b"r\0" as *const u8 as *const libc::c_char
        } else {
            b"rb\0" as *const u8 as *const libc::c_char
        },
    );
    fp = fopen(filename, mode.as_mut_ptr());
    if fp.is_null() {
        NOTIFY(
            ERROR,
            b"unable to open file: %s\0" as *const u8 as *const libc::c_char,
            filename,
        );
        return;
    }
    fseek(fp, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
    len = ftell(fp) as size_t;
    fseek(fp, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
    buf = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    if fread(buf as *mut libc::c_void, 1 as libc::c_int as libc::c_ulong, len, fp) == len
    {
        if is_ascii(filename) as u64 != 0 {
            *buf.offset(len as isize) = '\0' as i32 as libc::c_char;
            trim(buf);
            len = strlen(buf);
        }
    } else {
        NOTIFY(
            ERROR,
            b"unable to read file: %s\0" as *const u8 as *const libc::c_char,
            filename,
        );
    }
    fclose(fp);
    if len > 0 as libc::c_int as libc::c_ulong {
        if empty((my.conttype).as_mut_ptr()) as u64 == 0 {
            url_set_conttype(U, (my.conttype).as_mut_ptr());
        } else {
            url_set_conttype(U, get_content_type(filename));
        }
        url_set_postdata(U, buf, len);
    }
    xfree(buf as *mut libc::c_void);
}
pub unsafe extern "C" fn write_file(
    mut U: URL,
    mut buf: *mut libc::c_char,
    mut len: size_t,
) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut mode: [libc::c_char; 8] = [0; 8];
    memset(
        mode.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
    );
    snprintf(
        mode.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        if !(url_get_file(U)).is_null() {
            b"w\0" as *const u8 as *const libc::c_char
        } else {
            b"wb\0" as *const u8 as *const libc::c_char
        },
    );
    fp = fopen(url_get_file(U), mode.as_mut_ptr());
    if !fp.is_null() {
        fwrite(buf as *const libc::c_void, len, 1 as libc::c_int as libc::c_ulong, fp);
    } else {
        NOTIFY(ERROR, b"unable to write to file\0" as *const u8 as *const libc::c_char);
    }
    fclose(fp);
}
