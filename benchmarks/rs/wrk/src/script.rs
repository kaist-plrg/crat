use ::libc;
use ::c2rust_bitfields;
use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    pub type lua_State;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ssl_st;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> libc::c_int;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn getnameinfo(
        __sa: *const sockaddr,
        __salen: socklen_t,
        __host: *mut libc::c_char,
        __hostlen: socklen_t,
        __serv: *mut libc::c_char,
        __servlen: socklen_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn stats_value_at(stats: *mut stats, _: uint64_t, _: *mut uint64_t) -> uint64_t;
    fn stats_popcount(_: *mut stats) -> uint64_t;
    fn stats_percentile(_: *mut stats, _: f128::f128) -> uint64_t;
    fn stats_stdev(stats: *mut stats, _: f128::f128) -> f128::f128;
    fn stats_mean(_: *mut stats) -> f128::f128;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn luaL_pushresult(B: *mut luaL_Buffer);
    fn luaL_newstate() -> *mut lua_State;
    fn luaL_loadstring(L: *mut lua_State, s: *const libc::c_char) -> libc::c_int;
    fn luaL_loadfile(L: *mut lua_State, filename: *const libc::c_char) -> libc::c_int;
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn luaL_checkudata(
        L: *mut lua_State,
        ud: libc::c_int,
        tname: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn luaL_newmetatable(L: *mut lua_State, tname: *const libc::c_char) -> libc::c_int;
    fn luaL_checknumber(L: *mut lua_State, numArg: libc::c_int) -> lua_Number;
    fn luaL_argerror(
        L: *mut lua_State,
        numarg: libc::c_int,
        extramsg: *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_register(
        L: *mut lua_State,
        libname: *const libc::c_char,
        l: *const luaL_Reg,
    );
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn luaL_openlibs(L: *mut lua_State);
    fn lua_next(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_pcall(
        L: *mut lua_State,
        nargs: libc::c_int,
        nresults: libc::c_int,
        errfunc: libc::c_int,
    ) -> libc::c_int;
    fn lua_call(L: *mut lua_State, nargs: libc::c_int, nresults: libc::c_int);
    fn lua_setmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    fn lua_rawseti(L: *mut lua_State, idx: libc::c_int, n: libc::c_int);
    fn lua_rawset(L: *mut lua_State, idx: libc::c_int);
    fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    fn lua_settable(L: *mut lua_State, idx: libc::c_int);
    fn lua_newuserdata(L: *mut lua_State, sz: size_t) -> *mut libc::c_void;
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int);
    fn lua_getfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    fn lua_pushboolean(L: *mut lua_State, b: libc::c_int);
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: libc::c_int);
    fn lua_pushfstring(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        _: ...
    ) -> *const libc::c_char;
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char);
    fn lua_pushlstring(L: *mut lua_State, s: *const libc::c_char, l: size_t);
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    fn lua_pushnil(L: *mut lua_State);
    fn lua_objlen(L: *mut lua_State, idx: libc::c_int) -> size_t;
    fn lua_tolstring(
        L: *mut lua_State,
        idx: libc::c_int,
        len: *mut size_t,
    ) -> *const libc::c_char;
    fn lua_toboolean(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_tonumber(L: *mut lua_State, idx: libc::c_int) -> lua_Number;
    fn lua_typename(L: *mut lua_State, tp: libc::c_int) -> *const libc::c_char;
    fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_pushvalue(L: *mut lua_State, idx: libc::c_int);
    fn lua_settop(L: *mut lua_State, idx: libc::c_int);
    fn http_parser_parse_url(
        buf: *const libc::c_char,
        buflen: size_t,
        is_connect: libc::c_int,
        u: *mut http_parser_url,
    ) -> libc::c_int;
    fn http_errno_description(err: http_errno) -> *const libc::c_char;
    fn http_parser_execute(
        parser: *mut http_parser,
        settings: *const http_parser_settings,
        data: *const libc::c_char,
        len: size_t,
    ) -> size_t;
    fn http_parser_init(parser: *mut http_parser, type_0: http_parser_type);
    fn aeStop(eventLoop: *mut aeEventLoop);
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type time_t = __time_t;
pub type pthread_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub type lua_CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;
pub type lua_Number = libc::c_double;
pub type lua_Integer = ptrdiff_t;
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
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Buffer {
    pub p: *mut libc::c_char,
    pub lvl: libc::c_int,
    pub L: *mut lua_State,
    pub buffer: [libc::c_char; 8192],
}
pub type socklen_t = __socklen_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct errors {
    pub connect: uint32_t,
    pub read: uint32_t,
    pub write: uint32_t,
    pub status: uint32_t,
    pub timeout: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats {
    pub count: uint64_t,
    pub limit: uint64_t,
    pub min: uint64_t,
    pub max: uint64_t,
    pub data: [uint64_t; 0],
}
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
pub type SSL = ssl_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeEventLoop {
    pub maxfd: libc::c_int,
    pub setsize: libc::c_int,
    pub timeEventNextId: libc::c_longlong,
    pub lastTime: time_t,
    pub events: *mut aeFileEvent,
    pub fired: *mut aeFiredEvent,
    pub timeEventHead: *mut aeTimeEvent,
    pub stop: libc::c_int,
    pub apidata: *mut libc::c_void,
    pub beforesleep: Option::<aeBeforeSleepProc>,
}
pub type aeBeforeSleepProc = unsafe extern "C" fn(*mut aeEventLoop) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeTimeEvent {
    pub id: libc::c_longlong,
    pub when_sec: libc::c_long,
    pub when_ms: libc::c_long,
    pub timeProc: Option::<aeTimeProc>,
    pub finalizerProc: Option::<aeEventFinalizerProc>,
    pub clientData: *mut libc::c_void,
    pub next: *mut aeTimeEvent,
}
pub type aeEventFinalizerProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    *mut libc::c_void,
) -> ();
pub type aeTimeProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    libc::c_longlong,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeFiredEvent {
    pub fd: libc::c_int,
    pub mask: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeFileEvent {
    pub mask: libc::c_int,
    pub rfileProc: Option::<aeFileProc>,
    pub wfileProc: Option::<aeFileProc>,
    pub clientData: *mut libc::c_void,
}
pub type aeFileProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    libc::c_int,
    *mut libc::c_void,
    libc::c_int,
) -> ();
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct http_parser {
    #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "0..=1")]
    #[bitfield(name = "flags", ty = "libc::c_uint", bits = "2..=9")]
    #[bitfield(name = "state", ty = "libc::c_uint", bits = "10..=16")]
    #[bitfield(name = "header_state", ty = "libc::c_uint", bits = "17..=23")]
    #[bitfield(name = "index", ty = "libc::c_uint", bits = "24..=30")]
    #[bitfield(name = "lenient_http_headers", ty = "libc::c_uint", bits = "31..=31")]
    pub type_0_flags_state_header_state_index_lenient_http_headers: [u8; 4],
    pub nread: uint32_t,
    pub content_length: uint64_t,
    pub http_major: libc::c_ushort,
    pub http_minor: libc::c_ushort,
    #[bitfield(name = "status_code", ty = "libc::c_uint", bits = "0..=15")]
    #[bitfield(name = "method", ty = "libc::c_uint", bits = "16..=23")]
    #[bitfield(name = "http_errno", ty = "libc::c_uint", bits = "24..=30")]
    #[bitfield(name = "upgrade", ty = "libc::c_uint", bits = "31..=31")]
    pub status_code_method_http_errno_upgrade: [u8; 4],
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_parser_settings {
    pub on_message_begin: http_cb,
    pub on_url: http_data_cb,
    pub on_status: http_data_cb,
    pub on_header_field: http_data_cb,
    pub on_header_value: http_data_cb,
    pub on_headers_complete: http_cb,
    pub on_body: http_data_cb,
    pub on_message_complete: http_cb,
    pub on_chunk_header: http_cb,
    pub on_chunk_complete: http_cb,
}
pub type http_cb = Option::<unsafe extern "C" fn(*mut http_parser) -> libc::c_int>;
pub type http_data_cb = Option::<
    unsafe extern "C" fn(*mut http_parser, *const libc::c_char, size_t) -> libc::c_int,
>;
pub type http_parser_type = libc::c_uint;
pub const HTTP_BOTH: http_parser_type = 2;
pub const HTTP_RESPONSE: http_parser_type = 1;
pub const HTTP_REQUEST: http_parser_type = 0;
pub type http_errno = libc::c_uint;
pub const HPE_UNKNOWN: http_errno = 32;
pub const HPE_PAUSED: http_errno = 31;
pub const HPE_STRICT: http_errno = 30;
pub const HPE_INVALID_INTERNAL_STATE: http_errno = 29;
pub const HPE_INVALID_CONSTANT: http_errno = 28;
pub const HPE_INVALID_CHUNK_SIZE: http_errno = 27;
pub const HPE_UNEXPECTED_CONTENT_LENGTH: http_errno = 26;
pub const HPE_INVALID_CONTENT_LENGTH: http_errno = 25;
pub const HPE_INVALID_HEADER_TOKEN: http_errno = 24;
pub const HPE_LF_EXPECTED: http_errno = 23;
pub const HPE_INVALID_FRAGMENT: http_errno = 22;
pub const HPE_INVALID_QUERY_STRING: http_errno = 21;
pub const HPE_INVALID_PATH: http_errno = 20;
pub const HPE_INVALID_PORT: http_errno = 19;
pub const HPE_INVALID_HOST: http_errno = 18;
pub const HPE_INVALID_URL: http_errno = 17;
pub const HPE_INVALID_METHOD: http_errno = 16;
pub const HPE_INVALID_STATUS: http_errno = 15;
pub const HPE_INVALID_VERSION: http_errno = 14;
pub const HPE_CLOSED_CONNECTION: http_errno = 13;
pub const HPE_HEADER_OVERFLOW: http_errno = 12;
pub const HPE_INVALID_EOF_STATE: http_errno = 11;
pub const HPE_CB_chunk_complete: http_errno = 10;
pub const HPE_CB_chunk_header: http_errno = 9;
pub const HPE_CB_status: http_errno = 8;
pub const HPE_CB_message_complete: http_errno = 7;
pub const HPE_CB_body: http_errno = 6;
pub const HPE_CB_headers_complete: http_errno = 5;
pub const HPE_CB_header_value: http_errno = 4;
pub const HPE_CB_header_field: http_errno = 3;
pub const HPE_CB_url: http_errno = 2;
pub const HPE_CB_message_begin: http_errno = 1;
pub const HPE_OK: http_errno = 0;
pub type http_parser_url_fields = libc::c_uint;
pub const UF_MAX: http_parser_url_fields = 7;
pub const UF_USERINFO: http_parser_url_fields = 6;
pub const UF_FRAGMENT: http_parser_url_fields = 5;
pub const UF_QUERY: http_parser_url_fields = 4;
pub const UF_PATH: http_parser_url_fields = 3;
pub const UF_PORT: http_parser_url_fields = 2;
pub const UF_HOST: http_parser_url_fields = 1;
pub const UF_SCHEMA: http_parser_url_fields = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_parser_url {
    pub field_set: uint16_t,
    pub port: uint16_t,
    pub field_data: [C2RustUnnamed; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub off: uint16_t,
    pub len: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread {
    pub thread: pthread_t,
    pub loop_0: *mut aeEventLoop,
    pub addr: *mut addrinfo,
    pub connections: uint64_t,
    pub complete: uint64_t,
    pub requests: uint64_t,
    pub bytes: uint64_t,
    pub start: uint64_t,
    pub L: *mut lua_State,
    pub errors: errors,
    pub cs: *mut connection,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct connection {
    pub thread: *mut thread,
    pub parser: http_parser,
    pub state: C2RustUnnamed_0,
    pub fd: libc::c_int,
    pub ssl: *mut SSL,
    pub delayed: bool,
    pub start: uint64_t,
    pub request: *mut libc::c_char,
    pub length: size_t,
    pub written: size_t,
    pub pending: uint64_t,
    pub headers: buffer,
    pub body: buffer,
    pub buf: [libc::c_char; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub buffer: *mut libc::c_char,
    pub length: size_t,
    pub cursor: *mut libc::c_char,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const VALUE: C2RustUnnamed_0 = 1;
pub const FIELD: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_field {
    pub name: *mut libc::c_char,
    pub type_0: libc::c_int,
    pub value: *mut libc::c_void,
}
static mut addrlib: [luaL_Reg; 3] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"__tostring\0" as *const u8 as *const libc::c_char,
                func: Some(
                    script_addr_tostring
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__gc\0" as *const u8 as *const libc::c_char,
                func: Some(
                    script_addr_gc as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: 0 as *const libc::c_char,
                func: None,
            };
            init
        },
    ]
};
static mut statslib: [luaL_Reg; 4] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"__call\0" as *const u8 as *const libc::c_char,
                func: Some(
                    script_stats_call
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__index\0" as *const u8 as *const libc::c_char,
                func: Some(
                    script_stats_index
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__len\0" as *const u8 as *const libc::c_char,
                func: Some(
                    script_stats_len
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: 0 as *const libc::c_char,
                func: None,
            };
            init
        },
    ]
};
static mut threadlib: [luaL_Reg; 3] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"__index\0" as *const u8 as *const libc::c_char,
                func: Some(
                    script_thread_index
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__newindex\0" as *const u8 as *const libc::c_char,
                func: Some(
                    script_thread_newindex
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: 0 as *const libc::c_char,
                func: None,
            };
            init
        },
    ]
};
pub unsafe extern "C" fn script_create(
    mut file: *mut libc::c_char,
    mut url: *mut libc::c_char,
    mut headers: *mut *mut libc::c_char,
) -> *mut lua_State {
    let mut L: *mut lua_State = luaL_newstate();
    luaL_openlibs(L);
    (luaL_loadstring(L, b"wrk = require \"wrk\"\0" as *const u8 as *const libc::c_char)
        != 0
        || lua_pcall(L, 0 as libc::c_int, -(1 as libc::c_int), 0 as libc::c_int) != 0)
        as libc::c_int;
    luaL_newmetatable(L, b"wrk.addr\0" as *const u8 as *const libc::c_char);
    luaL_register(L, 0 as *const libc::c_char, addrlib.as_ptr());
    luaL_newmetatable(L, b"wrk.stats\0" as *const u8 as *const libc::c_char);
    luaL_register(L, 0 as *const libc::c_char, statslib.as_ptr());
    luaL_newmetatable(L, b"wrk.thread\0" as *const u8 as *const libc::c_char);
    luaL_register(L, 0 as *const libc::c_char, threadlib.as_ptr());
    let mut parts: http_parser_url = {
        let mut init = http_parser_url {
            field_set: 0,
            port: 0,
            field_data: [C2RustUnnamed { off: 0, len: 0 }; 7],
        };
        init
    };
    script_parse_url(url, &mut parts);
    let mut path: *mut libc::c_char = b"/\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    if parts.field_set as libc::c_int & (1 as libc::c_int) << UF_PATH as libc::c_int != 0
    {
        path = &mut *url
            .offset(
                (*(parts.field_data)
                    .as_mut_ptr()
                    .offset(UF_PATH as libc::c_int as isize))
                    .off as isize,
            ) as *mut libc::c_char;
    }
    let fields: [table_field; 4] = [
        {
            let mut init = table_field {
                name: b"lookup\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                type_0: 6 as libc::c_int,
                value: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>,
                    *mut libc::c_void,
                >(
                    Some(
                        script_wrk_lookup
                            as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = table_field {
                name: b"connect\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                type_0: 6 as libc::c_int,
                value: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>,
                    *mut libc::c_void,
                >(
                    Some(
                        script_wrk_connect
                            as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = table_field {
                name: b"path\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                type_0: 4 as libc::c_int,
                value: path as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = table_field {
                name: 0 as *mut libc::c_char,
                type_0: 0 as libc::c_int,
                value: 0 as *mut libc::c_void,
            };
            init
        },
    ];
    lua_getfield(
        L,
        -(10002 as libc::c_int),
        b"wrk\0" as *const u8 as *const libc::c_char,
    );
    set_field(
        L,
        4 as libc::c_int,
        b"scheme\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        push_url_part(L, url, &mut parts, UF_SCHEMA),
    );
    set_field(
        L,
        4 as libc::c_int,
        b"host\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        push_url_part(L, url, &mut parts, UF_HOST),
    );
    set_field(
        L,
        4 as libc::c_int,
        b"port\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        push_url_part(L, url, &mut parts, UF_PORT),
    );
    set_fields(L, 4 as libc::c_int, fields.as_ptr());
    lua_getfield(L, 4 as libc::c_int, b"headers\0" as *const u8 as *const libc::c_char);
    let mut h: *mut *mut libc::c_char = headers;
    while !(*h).is_null() {
        let mut p: *mut libc::c_char = strchr(*h, ':' as i32);
        if !p.is_null()
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == ' ' as i32
        {
            lua_pushlstring(L, *h, p.offset_from(*h) as libc::c_long as size_t);
            lua_pushstring(L, p.offset(2 as libc::c_int as isize));
            lua_settable(L, 5 as libc::c_int);
        }
        h = h.offset(1);
        h;
    }
    lua_settop(L, -(5 as libc::c_int) - 1 as libc::c_int);
    if !file.is_null()
        && (luaL_loadfile(L, file) != 0
            || lua_pcall(L, 0 as libc::c_int, -(1 as libc::c_int), 0 as libc::c_int)
                != 0)
    {
        let mut cause: *const libc::c_char = lua_tolstring(
            L,
            -(1 as libc::c_int),
            0 as *mut size_t,
        );
        fprintf(stderr, b"%s: %s\n\0" as *const u8 as *const libc::c_char, file, cause);
    }
    return L;
}
pub unsafe extern "C" fn script_resolve(
    mut L: *mut lua_State,
    mut host: *mut libc::c_char,
    mut service: *mut libc::c_char,
) -> bool {
    lua_getfield(
        L,
        -(10002 as libc::c_int),
        b"wrk\0" as *const u8 as *const libc::c_char,
    );
    lua_getfield(
        L,
        -(1 as libc::c_int),
        b"resolve\0" as *const u8 as *const libc::c_char,
    );
    lua_pushstring(L, host);
    lua_pushstring(L, service);
    lua_call(L, 2 as libc::c_int, 0 as libc::c_int);
    lua_getfield(L, -(1 as libc::c_int), b"addrs\0" as *const u8 as *const libc::c_char);
    let mut count: size_t = lua_objlen(L, -(1 as libc::c_int));
    lua_settop(L, -(2 as libc::c_int) - 1 as libc::c_int);
    return count > 0 as libc::c_int as libc::c_ulong;
}
pub unsafe extern "C" fn script_push_thread(mut L: *mut lua_State, mut t: *mut thread) {
    let mut ptr: *mut *mut thread = lua_newuserdata(
        L,
        ::std::mem::size_of::<*mut *mut thread>() as libc::c_ulong,
    ) as *mut *mut thread;
    *ptr = t;
    lua_getfield(
        L,
        -(10000 as libc::c_int),
        b"wrk.thread\0" as *const u8 as *const libc::c_char,
    );
    lua_setmetatable(L, -(2 as libc::c_int));
}
pub unsafe extern "C" fn script_init(
    mut L: *mut lua_State,
    mut t: *mut thread,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    lua_getfield(
        (*t).L,
        -(10002 as libc::c_int),
        b"wrk\0" as *const u8 as *const libc::c_char,
    );
    script_push_thread((*t).L, t);
    lua_setfield(
        (*t).L,
        -(2 as libc::c_int),
        b"thread\0" as *const u8 as *const libc::c_char,
    );
    lua_getfield(
        L,
        -(10002 as libc::c_int),
        b"wrk\0" as *const u8 as *const libc::c_char,
    );
    lua_getfield(L, -(1 as libc::c_int), b"setup\0" as *const u8 as *const libc::c_char);
    script_push_thread(L, t);
    lua_call(L, 1 as libc::c_int, 0 as libc::c_int);
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    lua_getfield(
        (*t).L,
        -(1 as libc::c_int),
        b"init\0" as *const u8 as *const libc::c_char,
    );
    lua_createtable((*t).L, 0 as libc::c_int, 0 as libc::c_int);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < argc {
        lua_pushstring((*t).L, *argv.offset(i as isize));
        lua_rawseti((*t).L, -(2 as libc::c_int), i);
        i += 1;
        i;
    }
    lua_call((*t).L, 1 as libc::c_int, 0 as libc::c_int);
    lua_settop((*t).L, -(1 as libc::c_int) - 1 as libc::c_int);
}
pub unsafe extern "C" fn script_delay(mut L: *mut lua_State) -> uint64_t {
    lua_getfield(
        L,
        -(10002 as libc::c_int),
        b"delay\0" as *const u8 as *const libc::c_char,
    );
    lua_call(L, 0 as libc::c_int, 1 as libc::c_int);
    let mut delay: uint64_t = lua_tonumber(L, -(1 as libc::c_int)) as uint64_t;
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return delay;
}
pub unsafe extern "C" fn script_request(
    mut L: *mut lua_State,
    mut buf: *mut *mut libc::c_char,
    mut len: *mut size_t,
) {
    let mut pop: libc::c_int = 1 as libc::c_int;
    lua_getfield(
        L,
        -(10002 as libc::c_int),
        b"request\0" as *const u8 as *const libc::c_char,
    );
    if !(lua_type(L, -(1 as libc::c_int)) == 6 as libc::c_int) {
        lua_getfield(
            L,
            -(10002 as libc::c_int),
            b"wrk\0" as *const u8 as *const libc::c_char,
        );
        lua_getfield(
            L,
            -(1 as libc::c_int),
            b"request\0" as *const u8 as *const libc::c_char,
        );
        pop += 2 as libc::c_int;
    }
    lua_call(L, 0 as libc::c_int, 1 as libc::c_int);
    let mut str: *const libc::c_char = lua_tolstring(L, -(1 as libc::c_int), len);
    *buf = realloc(*buf as *mut libc::c_void, *len) as *mut libc::c_char;
    memcpy(*buf as *mut libc::c_void, str as *const libc::c_void, *len);
    lua_settop(L, -pop - 1 as libc::c_int);
}
pub unsafe extern "C" fn script_response(
    mut L: *mut lua_State,
    mut status: libc::c_int,
    mut headers: *mut buffer,
    mut body: *mut buffer,
) {
    lua_getfield(
        L,
        -(10002 as libc::c_int),
        b"response\0" as *const u8 as *const libc::c_char,
    );
    lua_pushinteger(L, status as lua_Integer);
    lua_createtable(L, 0 as libc::c_int, 0 as libc::c_int);
    let mut c: *mut libc::c_char = (*headers).buffer;
    while c < (*headers).cursor {
        c = buffer_pushlstring(L, c);
        c = buffer_pushlstring(L, c);
        lua_rawset(L, -(3 as libc::c_int));
    }
    lua_pushlstring(
        L,
        (*body).buffer,
        ((*body).cursor).offset_from((*body).buffer) as libc::c_long as size_t,
    );
    lua_call(L, 3 as libc::c_int, 0 as libc::c_int);
    buffer_reset(headers);
    buffer_reset(body);
}
pub unsafe extern "C" fn script_is_function(
    mut L: *mut lua_State,
    mut name: *mut libc::c_char,
) -> bool {
    lua_getfield(L, -(10002 as libc::c_int), name);
    let mut is_function: bool = lua_type(L, -(1 as libc::c_int)) == 6 as libc::c_int;
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return is_function;
}
pub unsafe extern "C" fn script_is_static(mut L: *mut lua_State) -> bool {
    return !script_is_function(
        L,
        b"request\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn script_want_response(mut L: *mut lua_State) -> bool {
    return script_is_function(
        L,
        b"response\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn script_has_delay(mut L: *mut lua_State) -> bool {
    return script_is_function(
        L,
        b"delay\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn script_has_done(mut L: *mut lua_State) -> bool {
    return script_is_function(
        L,
        b"done\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn script_header_done(
    mut L: *mut lua_State,
    mut buffer: *mut luaL_Buffer,
) {
    luaL_pushresult(buffer);
}
pub unsafe extern "C" fn script_summary(
    mut L: *mut lua_State,
    mut duration: uint64_t,
    mut requests: uint64_t,
    mut bytes: uint64_t,
) {
    let fields: [table_field; 4] = [
        {
            let mut init = table_field {
                name: b"duration\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                type_0: 3 as libc::c_int,
                value: &mut duration as *mut uint64_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = table_field {
                name: b"requests\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                type_0: 3 as libc::c_int,
                value: &mut requests as *mut uint64_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = table_field {
                name: b"bytes\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                type_0: 3 as libc::c_int,
                value: &mut bytes as *mut uint64_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = table_field {
                name: 0 as *mut libc::c_char,
                type_0: 0 as libc::c_int,
                value: 0 as *mut libc::c_void,
            };
            init
        },
    ];
    lua_createtable(L, 0 as libc::c_int, 0 as libc::c_int);
    set_fields(L, 1 as libc::c_int, fields.as_ptr());
}
pub unsafe extern "C" fn script_errors(mut L: *mut lua_State, mut errors: *mut errors) {
    let mut e: [uint64_t; 5] = [
        (*errors).connect as uint64_t,
        (*errors).read as uint64_t,
        (*errors).write as uint64_t,
        (*errors).status as uint64_t,
        (*errors).timeout as uint64_t,
    ];
    let fields: [table_field; 6] = [
        {
            let mut init = table_field {
                name: b"connect\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                type_0: 3 as libc::c_int,
                value: &mut *e.as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut uint64_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = table_field {
                name: b"read\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                type_0: 3 as libc::c_int,
                value: &mut *e.as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut uint64_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = table_field {
                name: b"write\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                type_0: 3 as libc::c_int,
                value: &mut *e.as_mut_ptr().offset(2 as libc::c_int as isize)
                    as *mut uint64_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = table_field {
                name: b"status\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                type_0: 3 as libc::c_int,
                value: &mut *e.as_mut_ptr().offset(3 as libc::c_int as isize)
                    as *mut uint64_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = table_field {
                name: b"timeout\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                type_0: 3 as libc::c_int,
                value: &mut *e.as_mut_ptr().offset(4 as libc::c_int as isize)
                    as *mut uint64_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = table_field {
                name: 0 as *mut libc::c_char,
                type_0: 0 as libc::c_int,
                value: 0 as *mut libc::c_void,
            };
            init
        },
    ];
    lua_createtable(L, 0 as libc::c_int, 0 as libc::c_int);
    set_fields(L, 2 as libc::c_int, fields.as_ptr());
    lua_setfield(L, 1 as libc::c_int, b"errors\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn script_push_stats(mut L: *mut lua_State, mut s: *mut stats) {
    let mut ptr: *mut *mut stats = lua_newuserdata(
        L,
        ::std::mem::size_of::<*mut *mut stats>() as libc::c_ulong,
    ) as *mut *mut stats;
    *ptr = s;
    lua_getfield(
        L,
        -(10000 as libc::c_int),
        b"wrk.stats\0" as *const u8 as *const libc::c_char,
    );
    lua_setmetatable(L, -(2 as libc::c_int));
}
pub unsafe extern "C" fn script_done(
    mut L: *mut lua_State,
    mut latency: *mut stats,
    mut requests: *mut stats,
) {
    lua_getfield(
        L,
        -(10002 as libc::c_int),
        b"done\0" as *const u8 as *const libc::c_char,
    );
    lua_pushvalue(L, 1 as libc::c_int);
    script_push_stats(L, latency);
    script_push_stats(L, requests);
    lua_call(L, 3 as libc::c_int, 0 as libc::c_int);
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
}
unsafe extern "C" fn verify_request(mut parser: *mut http_parser) -> libc::c_int {
    let mut count: *mut size_t = (*parser).data as *mut size_t;
    *count = (*count).wrapping_add(1);
    *count;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn script_verify_request(mut L: *mut lua_State) -> size_t {
    let mut settings: http_parser_settings = {
        let mut init = http_parser_settings {
            on_message_begin: None,
            on_url: None,
            on_status: None,
            on_header_field: None,
            on_header_value: None,
            on_headers_complete: None,
            on_body: None,
            on_message_complete: Some(
                verify_request as unsafe extern "C" fn(*mut http_parser) -> libc::c_int,
            ),
            on_chunk_header: None,
            on_chunk_complete: None,
        };
        init
    };
    let mut parser: http_parser = http_parser {
        type_0_flags_state_header_state_index_lenient_http_headers: [0; 4],
        nread: 0,
        content_length: 0,
        http_major: 0,
        http_minor: 0,
        status_code_method_http_errno_upgrade: [0; 4],
        data: 0 as *mut libc::c_void,
    };
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut count: size_t = 0 as libc::c_int as size_t;
    script_request(L, &mut request, &mut len);
    http_parser_init(&mut parser, HTTP_REQUEST);
    parser.data = &mut count as *mut size_t as *mut libc::c_void;
    let mut parsed: size_t = http_parser_execute(
        &mut parser,
        &mut settings,
        request,
        len,
    );
    if parsed != len || count == 0 as libc::c_int as libc::c_ulong {
        let mut err: http_errno = parser.http_errno() as http_errno;
        let mut desc: *const libc::c_char = http_errno_description(err);
        let mut msg: *const libc::c_char = if err as libc::c_uint
            != HPE_OK as libc::c_int as libc::c_uint
        {
            desc
        } else {
            b"incomplete request\0" as *const u8 as *const libc::c_char
        };
        let mut line: libc::c_int = 1 as libc::c_int;
        let mut column: libc::c_int = 1 as libc::c_int;
        let mut c: *mut libc::c_char = request;
        while c < request.offset(parsed as isize) {
            column += 1;
            column;
            if *c as libc::c_int == '\n' as i32 {
                column = 1 as libc::c_int;
                line += 1;
                line;
            }
            c = c.offset(1);
            c;
        }
        fprintf(
            stderr,
            b"%s at %d:%d\n\0" as *const u8 as *const libc::c_char,
            msg,
            line,
            column,
        );
        exit(1 as libc::c_int);
    }
    return count;
}
unsafe extern "C" fn checkaddr(mut L: *mut lua_State) -> *mut addrinfo {
    let mut addr: *mut addrinfo = luaL_checkudata(
        L,
        -(1 as libc::c_int),
        b"wrk.addr\0" as *const u8 as *const libc::c_char,
    ) as *mut addrinfo;
    (!addr.is_null()
        || luaL_argerror(
            L,
            1 as libc::c_int,
            b"`addr' expected\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    return addr;
}
pub unsafe extern "C" fn script_addr_copy(
    mut src: *mut addrinfo,
    mut dst: *mut addrinfo,
) {
    *dst = *src;
    (*dst).ai_addr = zmalloc((*src).ai_addrlen as size_t) as *mut sockaddr;
    memcpy(
        (*dst).ai_addr as *mut libc::c_void,
        (*src).ai_addr as *const libc::c_void,
        (*src).ai_addrlen as libc::c_ulong,
    );
}
pub unsafe extern "C" fn script_addr_clone(
    mut L: *mut lua_State,
    mut addr: *mut addrinfo,
) -> *mut addrinfo {
    let mut udata: *mut addrinfo = lua_newuserdata(
        L,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    ) as *mut addrinfo;
    lua_getfield(
        L,
        -(10000 as libc::c_int),
        b"wrk.addr\0" as *const u8 as *const libc::c_char,
    );
    lua_setmetatable(L, -(2 as libc::c_int));
    script_addr_copy(addr, udata);
    return udata;
}
unsafe extern "C" fn script_addr_tostring(mut L: *mut lua_State) -> libc::c_int {
    let mut addr: *mut addrinfo = checkaddr(L);
    let mut host: [libc::c_char; 1025] = [0; 1025];
    let mut service: [libc::c_char; 32] = [0; 32];
    let mut flags: libc::c_int = 1 as libc::c_int | 2 as libc::c_int;
    let mut rc: libc::c_int = getnameinfo(
        (*addr).ai_addr,
        (*addr).ai_addrlen,
        host.as_mut_ptr(),
        1025 as libc::c_int as socklen_t,
        service.as_mut_ptr(),
        32 as libc::c_int as socklen_t,
        flags,
    );
    if rc != 0 as libc::c_int {
        let mut msg: *const libc::c_char = gai_strerror(rc);
        return luaL_error(
            L,
            b"addr tostring failed %s\0" as *const u8 as *const libc::c_char,
            msg,
        );
    }
    lua_pushfstring(
        L,
        b"%s:%s\0" as *const u8 as *const libc::c_char,
        host.as_mut_ptr(),
        service.as_mut_ptr(),
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn script_addr_gc(mut L: *mut lua_State) -> libc::c_int {
    let mut addr: *mut addrinfo = checkaddr(L);
    zfree((*addr).ai_addr as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn checkstats(mut L: *mut lua_State) -> *mut stats {
    let mut s: *mut *mut stats = luaL_checkudata(
        L,
        1 as libc::c_int,
        b"wrk.stats\0" as *const u8 as *const libc::c_char,
    ) as *mut *mut stats;
    (!s.is_null()
        || luaL_argerror(
            L,
            1 as libc::c_int,
            b"`stats' expected\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    return *s;
}
unsafe extern "C" fn script_stats_percentile(mut L: *mut lua_State) -> libc::c_int {
    let mut s: *mut stats = checkstats(L);
    let mut p: lua_Number = luaL_checknumber(L, 2 as libc::c_int);
    lua_pushnumber(L, stats_percentile(s, f128::f128::new(p)) as lua_Number);
    return 1 as libc::c_int;
}
unsafe extern "C" fn script_stats_call(mut L: *mut lua_State) -> libc::c_int {
    let mut s: *mut stats = checkstats(L);
    let mut index: uint64_t = lua_tonumber(L, 2 as libc::c_int) as uint64_t;
    let mut count: uint64_t = 0;
    lua_pushnumber(
        L,
        stats_value_at(
            s,
            index.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            &mut count,
        ) as lua_Number,
    );
    lua_pushnumber(L, count as lua_Number);
    return 2 as libc::c_int;
}
unsafe extern "C" fn script_stats_index(mut L: *mut lua_State) -> libc::c_int {
    let mut s: *mut stats = checkstats(L);
    let mut method: *const libc::c_char = lua_tolstring(
        L,
        2 as libc::c_int,
        0 as *mut size_t,
    );
    if strcmp(b"min\0" as *const u8 as *const libc::c_char, method) == 0 {
        lua_pushnumber(L, (*s).min as lua_Number);
    }
    if strcmp(b"max\0" as *const u8 as *const libc::c_char, method) == 0 {
        lua_pushnumber(L, (*s).max as lua_Number);
    }
    if strcmp(b"mean\0" as *const u8 as *const libc::c_char, method) == 0 {
        lua_pushnumber(L, (stats_mean(s)).to_f64().unwrap());
    }
    if strcmp(b"stdev\0" as *const u8 as *const libc::c_char, method) == 0 {
        lua_pushnumber(L, (stats_stdev(s, stats_mean(s))).to_f64().unwrap());
    }
    if strcmp(b"percentile\0" as *const u8 as *const libc::c_char, method) == 0 {
        lua_pushcclosure(
            L,
            Some(
                script_stats_percentile
                    as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
            ),
            0 as libc::c_int,
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn script_stats_len(mut L: *mut lua_State) -> libc::c_int {
    let mut s: *mut stats = checkstats(L);
    lua_pushinteger(L, stats_popcount(s) as lua_Integer);
    return 1 as libc::c_int;
}
unsafe extern "C" fn checkthread(mut L: *mut lua_State) -> *mut thread {
    let mut t: *mut *mut thread = luaL_checkudata(
        L,
        1 as libc::c_int,
        b"wrk.thread\0" as *const u8 as *const libc::c_char,
    ) as *mut *mut thread;
    (!t.is_null()
        || luaL_argerror(
            L,
            1 as libc::c_int,
            b"`thread' expected\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    return *t;
}
unsafe extern "C" fn script_thread_get(mut L: *mut lua_State) -> libc::c_int {
    let mut t: *mut thread = checkthread(L);
    let mut key: *const libc::c_char = lua_tolstring(
        L,
        -(1 as libc::c_int),
        0 as *mut size_t,
    );
    lua_getfield((*t).L, -(10002 as libc::c_int), key);
    script_copy_value((*t).L, L, -(1 as libc::c_int));
    lua_settop((*t).L, -(1 as libc::c_int) - 1 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn script_thread_set(mut L: *mut lua_State) -> libc::c_int {
    let mut t: *mut thread = checkthread(L);
    let mut name: *const libc::c_char = lua_tolstring(
        L,
        -(2 as libc::c_int),
        0 as *mut size_t,
    );
    script_copy_value(L, (*t).L, -(1 as libc::c_int));
    lua_setfield((*t).L, -(10002 as libc::c_int), name);
    return 0 as libc::c_int;
}
unsafe extern "C" fn script_thread_stop(mut L: *mut lua_State) -> libc::c_int {
    let mut t: *mut thread = checkthread(L);
    aeStop((*t).loop_0);
    return 0 as libc::c_int;
}
unsafe extern "C" fn script_thread_index(mut L: *mut lua_State) -> libc::c_int {
    let mut t: *mut thread = checkthread(L);
    let mut key: *const libc::c_char = lua_tolstring(
        L,
        2 as libc::c_int,
        0 as *mut size_t,
    );
    if strcmp(b"get\0" as *const u8 as *const libc::c_char, key) == 0 {
        lua_pushcclosure(
            L,
            Some(
                script_thread_get as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
            ),
            0 as libc::c_int,
        );
    }
    if strcmp(b"set\0" as *const u8 as *const libc::c_char, key) == 0 {
        lua_pushcclosure(
            L,
            Some(
                script_thread_set as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
            ),
            0 as libc::c_int,
        );
    }
    if strcmp(b"stop\0" as *const u8 as *const libc::c_char, key) == 0 {
        lua_pushcclosure(
            L,
            Some(
                script_thread_stop as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
            ),
            0 as libc::c_int,
        );
    }
    if strcmp(b"addr\0" as *const u8 as *const libc::c_char, key) == 0 {
        script_addr_clone(L, (*t).addr);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn script_thread_newindex(mut L: *mut lua_State) -> libc::c_int {
    let mut t: *mut thread = checkthread(L);
    let mut key: *const libc::c_char = lua_tolstring(
        L,
        -(2 as libc::c_int),
        0 as *mut size_t,
    );
    if strcmp(b"addr\0" as *const u8 as *const libc::c_char, key) == 0 {
        let mut addr: *mut addrinfo = checkaddr(L);
        if !((*t).addr).is_null() {
            zfree((*(*t).addr).ai_addr as *mut libc::c_void);
        }
        (*t)
            .addr = zrealloc(
            (*t).addr as *mut libc::c_void,
            ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
        ) as *mut addrinfo;
        script_addr_copy(addr, (*t).addr);
    } else {
        luaL_error(
            L,
            b"cannot set '%s' on thread\0" as *const u8 as *const libc::c_char,
            lua_typename(L, lua_type(L, -(1 as libc::c_int))),
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn script_wrk_lookup(mut L: *mut lua_State) -> libc::c_int {
    let mut addrs: *mut addrinfo = 0 as *mut addrinfo;
    let mut hints: addrinfo = {
        let mut init = addrinfo {
            ai_flags: 0,
            ai_family: 0 as libc::c_int,
            ai_socktype: SOCK_STREAM as libc::c_int,
            ai_protocol: 0,
            ai_addrlen: 0,
            ai_addr: 0 as *mut sockaddr,
            ai_canonname: 0 as *mut libc::c_char,
            ai_next: 0 as *mut addrinfo,
        };
        init
    };
    let mut rc: libc::c_int = 0;
    let mut index: libc::c_int = 1 as libc::c_int;
    let mut host: *const libc::c_char = lua_tolstring(
        L,
        -(2 as libc::c_int),
        0 as *mut size_t,
    );
    let mut service: *const libc::c_char = lua_tolstring(
        L,
        -(1 as libc::c_int),
        0 as *mut size_t,
    );
    rc = getaddrinfo(host, service, &mut hints, &mut addrs);
    if rc != 0 as libc::c_int {
        let mut msg: *const libc::c_char = gai_strerror(rc);
        fprintf(
            stderr,
            b"unable to resolve %s:%s %s\n\0" as *const u8 as *const libc::c_char,
            host,
            service,
            msg,
        );
        exit(1 as libc::c_int);
    }
    lua_createtable(L, 0 as libc::c_int, 0 as libc::c_int);
    let mut addr: *mut addrinfo = addrs;
    while !addr.is_null() {
        script_addr_clone(L, addr);
        let fresh0 = index;
        index = index + 1;
        lua_rawseti(L, -(2 as libc::c_int), fresh0);
        addr = (*addr).ai_next;
    }
    freeaddrinfo(addrs);
    return 1 as libc::c_int;
}
unsafe extern "C" fn script_wrk_connect(mut L: *mut lua_State) -> libc::c_int {
    let mut addr: *mut addrinfo = checkaddr(L);
    let mut fd: libc::c_int = 0;
    let mut connected: libc::c_int = 0 as libc::c_int;
    fd = socket((*addr).ai_family, (*addr).ai_socktype, (*addr).ai_protocol);
    if fd != -(1 as libc::c_int) {
        connected = (connect(fd, (*addr).ai_addr, (*addr).ai_addrlen)
            == 0 as libc::c_int) as libc::c_int;
        close(fd);
    }
    lua_pushboolean(L, connected);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn script_copy_value(
    mut src: *mut lua_State,
    mut dst: *mut lua_State,
    mut index: libc::c_int,
) {
    match lua_type(src, index) {
        1 => {
            lua_pushboolean(dst, lua_toboolean(src, index));
        }
        0 => {
            lua_pushnil(dst);
        }
        3 => {
            lua_pushnumber(dst, lua_tonumber(src, index));
        }
        4 => {
            lua_pushstring(dst, lua_tolstring(src, index, 0 as *mut size_t));
        }
        5 => {
            lua_createtable(dst, 0 as libc::c_int, 0 as libc::c_int);
            lua_pushnil(src);
            while lua_next(src, index - 1 as libc::c_int) != 0 {
                script_copy_value(src, dst, -(2 as libc::c_int));
                script_copy_value(src, dst, -(1 as libc::c_int));
                lua_settable(dst, -(3 as libc::c_int));
                lua_settop(src, -(1 as libc::c_int) - 1 as libc::c_int);
            }
            lua_settop(src, -(1 as libc::c_int) - 1 as libc::c_int);
        }
        _ => {
            luaL_error(
                src,
                b"cannot transfer '%s' to thread\0" as *const u8 as *const libc::c_char,
                lua_typename(src, lua_type(src, index)),
            );
        }
    };
}
pub unsafe extern "C" fn script_parse_url(
    mut url: *mut libc::c_char,
    mut parts: *mut http_parser_url,
) -> libc::c_int {
    if http_parser_parse_url(url, strlen(url), 0 as libc::c_int, parts) == 0 {
        if (*parts).field_set as libc::c_int
            & (1 as libc::c_int) << UF_SCHEMA as libc::c_int == 0
        {
            return 0 as libc::c_int;
        }
        if (*parts).field_set as libc::c_int
            & (1 as libc::c_int) << UF_HOST as libc::c_int == 0
        {
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn push_url_part(
    mut L: *mut lua_State,
    mut url: *mut libc::c_char,
    mut parts: *mut http_parser_url,
    mut field: http_parser_url_fields,
) -> libc::c_int {
    let mut type_0: libc::c_int = if (*parts).field_set as libc::c_int
        & (1 as libc::c_int) << field as libc::c_uint != 0
    {
        4 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut off: uint16_t = 0;
    let mut len: uint16_t = 0;
    match type_0 {
        4 => {
            off = (*parts).field_data[field as usize].off;
            len = (*parts).field_data[field as usize].len;
            lua_pushlstring(L, &mut *url.offset(off as isize), len as size_t);
        }
        0 => {
            lua_pushnil(L);
        }
        _ => {}
    }
    return type_0;
}
unsafe extern "C" fn set_field(
    mut L: *mut lua_State,
    mut index: libc::c_int,
    mut field: *mut libc::c_char,
    mut type_0: libc::c_int,
) {
    lua_setfield(L, index, field);
}
unsafe extern "C" fn set_fields(
    mut L: *mut lua_State,
    mut index: libc::c_int,
    mut fields: *const table_field,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while !((*fields.offset(i as isize)).name).is_null() {
        let mut f: table_field = *fields.offset(i as isize);
        match if (f.value).is_null() { 0 as libc::c_int } else { f.type_0 } {
            6 => {
                lua_pushcclosure(
                    L,
                    ::std::mem::transmute::<*mut libc::c_void, lua_CFunction>(f.value),
                    0 as libc::c_int,
                );
            }
            3 => {
                lua_pushinteger(L, *(f.value as *mut lua_Integer));
            }
            4 => {
                lua_pushstring(L, f.value as *const libc::c_char);
            }
            0 => {
                lua_pushnil(L);
            }
            _ => {}
        }
        lua_setfield(L, index, f.name);
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn buffer_append(
    mut b: *mut buffer,
    mut data: *const libc::c_char,
    mut len: size_t,
) {
    let mut used: size_t = ((*b).cursor).offset_from((*b).buffer) as libc::c_long
        as size_t;
    while used.wrapping_add(len).wrapping_add(1 as libc::c_int as libc::c_ulong)
        >= (*b).length
    {
        (*b)
            .length = ((*b).length as libc::c_ulong)
            .wrapping_add(1024 as libc::c_int as libc::c_ulong) as size_t as size_t;
        (*b)
            .buffer = realloc((*b).buffer as *mut libc::c_void, (*b).length)
            as *mut libc::c_char;
        (*b).cursor = ((*b).buffer).offset(used as isize);
    }
    memcpy((*b).cursor as *mut libc::c_void, data as *const libc::c_void, len);
    (*b).cursor = ((*b).cursor).offset(len as isize);
}
pub unsafe extern "C" fn buffer_reset(mut b: *mut buffer) {
    (*b).cursor = (*b).buffer;
}
pub unsafe extern "C" fn buffer_pushlstring(
    mut L: *mut lua_State,
    mut start: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut end: *mut libc::c_char = strchr(start, 0 as libc::c_int);
    lua_pushlstring(L, start, end.offset_from(start) as libc::c_long as size_t);
    return end.offset(1 as libc::c_int as isize);
}
