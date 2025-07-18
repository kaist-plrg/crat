use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type epoll_event;
    pub type conf;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn array_push(a: *mut array) -> *mut libc::c_void;
    fn array_get(a: *const array, idx: uint32_t) -> *mut libc::c_void;
    fn _nc_alloc(
        size: size_t,
        name: *const libc::c_char,
        line: libc::c_int,
    ) -> *mut libc::c_void;
    fn _nc_zalloc(
        size: size_t,
        name: *const libc::c_char,
        line: libc::c_int,
    ) -> *mut libc::c_void;
    fn _nc_free(ptr: *mut libc::c_void, name: *const libc::c_char, line: libc::c_int);
    fn log_loggable(level: libc::c_int) -> libc::c_int;
    fn _log(
        file: *const libc::c_char,
        line: libc::c_int,
        panic: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn mbuf_get() -> *mut mbuf;
    fn mbuf_put(mbuf: *mut mbuf);
    fn mbuf_length(mbuf: *const mbuf) -> uint32_t;
    fn mbuf_insert(mhdr: *mut mhdr, mbuf: *mut mbuf);
    fn mbuf_remove(mhdr: *mut mhdr, mbuf: *mut mbuf);
    fn mbuf_copy(mbuf: *mut mbuf, pos: *const uint8_t, n: size_t);
    fn msg_set_placeholder_key(r: *mut msg) -> bool;
    fn msg_append(msg: *mut msg, pos: *const uint8_t, n: size_t) -> rstatus_t;
    fn msg_prepend(msg: *mut msg, pos: *const uint8_t, n: size_t) -> rstatus_t;
    fn msg_ensure_mbuf(msg: *mut msg, len: size_t) -> *mut mbuf;
    fn msg_backend_idx(
        msg: *const msg,
        key: *const uint8_t,
        keylen: uint32_t,
    ) -> uint32_t;
    fn msg_get(conn: *mut conn, request: bool, redis: bool) -> *mut msg;
    fn msg_gen_frag_id() -> uint64_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __socklen_t = libc::c_uint;
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
pub type rstatus_t = libc::c_int;
pub type err_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array {
    pub nelem: uint32_t,
    pub elem: *mut libc::c_void,
    pub size: size_t,
    pub nalloc: uint32_t,
}
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string {
    pub len: uint32_t,
    pub data: *mut uint8_t,
}
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context {
    pub id: uint32_t,
    pub cf: *mut conf,
    pub stats: *mut stats,
    pub pool: array,
    pub evb: *mut event_base,
    pub max_timeout: libc::c_int,
    pub timeout: libc::c_int,
    pub max_nfd: uint32_t,
    pub max_ncconn: uint32_t,
    pub max_nsconn: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_base {
    pub ep: libc::c_int,
    pub event: *mut epoll_event,
    pub nevent: libc::c_int,
    pub cb: event_cb_t,
}
pub type event_cb_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, uint32_t) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats {
    pub port: uint16_t,
    pub interval: libc::c_int,
    pub addr: string,
    pub start_ts: int64_t,
    pub buf: stats_buffer,
    pub current: array,
    pub shadow: array,
    pub sum: array,
    pub tid: pthread_t,
    pub sd: libc::c_int,
    pub service_str: string,
    pub service: string,
    pub source_str: string,
    pub source: string,
    pub version_str: string,
    pub version: string,
    pub uptime_str: string,
    pub timestamp_str: string,
    pub ntotal_conn_str: string,
    pub ncurr_conn_str: string,
    pub aggregate: libc::c_int,
    pub updated: libc::c_int,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_buffer {
    pub len: size_t,
    pub data: *mut uint8_t,
    pub size: size_t,
}
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct conn {
    pub conn_tqe: C2RustUnnamed_6,
    pub owner: *mut libc::c_void,
    pub sd: libc::c_int,
    pub family: libc::c_int,
    pub addrlen: socklen_t,
    pub addr: *mut sockaddr,
    pub imsg_q: msg_tqh,
    pub omsg_q: msg_tqh,
    pub rmsg: *mut msg,
    pub smsg: *mut msg,
    pub recv: conn_recv_t,
    pub recv_next: conn_recv_next_t,
    pub recv_done: conn_recv_done_t,
    pub send: conn_send_t,
    pub send_next: conn_send_next_t,
    pub send_done: conn_send_done_t,
    pub close: conn_close_t,
    pub active: conn_active_t,
    pub post_connect: conn_post_connect_t,
    pub swallow_msg: conn_swallow_msg_t,
    pub ref_0: conn_ref_t,
    pub unref: conn_unref_t,
    pub enqueue_inq: conn_msgq_t,
    pub dequeue_inq: conn_msgq_t,
    pub enqueue_outq: conn_msgq_t,
    pub dequeue_outq: conn_msgq_t,
    pub recv_bytes: size_t,
    pub send_bytes: size_t,
    pub events: uint32_t,
    pub err: err_t,
    #[bitfield(name = "recv_active", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "recv_ready", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "send_active", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "send_ready", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "client", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "proxy", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "connecting", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "connected", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "eof", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "done", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "redis", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "authenticated", ty = "libc::c_uint", bits = "11..=11")]
    pub recv_active_recv_ready_send_active_send_ready_client_proxy_connecting_connected_eof_done_redis_authenticated: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
pub type conn_msgq_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut msg) -> (),
>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct msg {
    pub c_tqe: C2RustUnnamed_3,
    pub s_tqe: C2RustUnnamed_2,
    pub m_tqe: C2RustUnnamed_1,
    pub id: uint64_t,
    pub peer: *mut msg,
    pub owner: *mut conn,
    pub tmo_rbe: rbnode,
    pub mhdr: mhdr,
    pub mlen: uint32_t,
    pub start_ts: int64_t,
    pub state: libc::c_int,
    pub pos: *mut uint8_t,
    pub token: *mut uint8_t,
    pub parser: msg_parse_t,
    pub result: msg_parse_result_t,
    pub fragment: msg_fragment_t,
    pub reply: msg_reply_t,
    pub add_auth: msg_add_auth_t,
    pub failure: msg_failure_t,
    pub pre_coalesce: msg_coalesce_t,
    pub post_coalesce: msg_coalesce_t,
    pub type_0: msg_type_t,
    pub keys: *mut array,
    pub vlen: uint32_t,
    pub end: *mut uint8_t,
    pub narg_start: *mut uint8_t,
    pub narg_end: *mut uint8_t,
    pub narg: uint32_t,
    pub rnarg: uint32_t,
    pub rlen: uint32_t,
    pub integer: uint32_t,
    pub is_top_level: uint8_t,
    pub frag_owner: *mut msg,
    pub nfrag: uint32_t,
    pub nfrag_done: uint32_t,
    pub frag_id: uint64_t,
    pub frag_seq: *mut *mut msg,
    pub err: err_t,
    #[bitfield(name = "error", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "ferror", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "request", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "quit", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "noreply", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "noforward", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "done", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "fdone", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "swallow", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "redis", ty = "libc::c_uint", bits = "9..=9")]
    pub error_ferror_request_quit_noreply_noforward_done_fdone_swallow_redis: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
}
pub type uint64_t = __uint64_t;
pub type msg_type_t = msg_type;
pub type msg_type = libc::c_uint;
pub const MSG_SENTINEL: msg_type = 184;
pub const MSG_RSP_REDIS_MULTIBULK: msg_type = 183;
pub const MSG_RSP_REDIS_BULK: msg_type = 182;
pub const MSG_RSP_REDIS_INTEGER: msg_type = 181;
pub const MSG_RSP_REDIS_ERROR_NOREPLICAS: msg_type = 180;
pub const MSG_RSP_REDIS_ERROR_MASTERDOWN: msg_type = 179;
pub const MSG_RSP_REDIS_ERROR_EXECABORT: msg_type = 178;
pub const MSG_RSP_REDIS_ERROR_WRONGTYPE: msg_type = 177;
pub const MSG_RSP_REDIS_ERROR_READONLY: msg_type = 176;
pub const MSG_RSP_REDIS_ERROR_NOSCRIPT: msg_type = 175;
pub const MSG_RSP_REDIS_ERROR_MISCONF: msg_type = 174;
pub const MSG_RSP_REDIS_ERROR_BUSYKEY: msg_type = 173;
pub const MSG_RSP_REDIS_ERROR_LOADING: msg_type = 172;
pub const MSG_RSP_REDIS_ERROR_NOAUTH: msg_type = 171;
pub const MSG_RSP_REDIS_ERROR_BUSY: msg_type = 170;
pub const MSG_RSP_REDIS_ERROR_OOM: msg_type = 169;
pub const MSG_RSP_REDIS_ERROR_ERR: msg_type = 168;
pub const MSG_RSP_REDIS_ERROR: msg_type = 167;
pub const MSG_RSP_REDIS_STATUS: msg_type = 166;
pub const MSG_REQ_REDIS_LOLWUT: msg_type = 165;
pub const MSG_REQ_REDIS_COMMAND: msg_type = 164;
pub const MSG_REQ_REDIS_SELECT: msg_type = 163;
pub const MSG_REQ_REDIS_AUTH: msg_type = 162;
pub const MSG_REQ_REDIS_QUIT: msg_type = 161;
pub const MSG_REQ_REDIS_PING: msg_type = 160;
pub const MSG_REQ_REDIS_EVALSHA: msg_type = 159;
pub const MSG_REQ_REDIS_EVAL: msg_type = 158;
pub const MSG_REQ_REDIS_GEOSEARCHSTORE: msg_type = 157;
pub const MSG_REQ_REDIS_GEOSEARCH: msg_type = 156;
pub const MSG_REQ_REDIS_GEOPOS: msg_type = 155;
pub const MSG_REQ_REDIS_GEORADIUSBYMEMBER: msg_type = 154;
pub const MSG_REQ_REDIS_GEORADIUS: msg_type = 153;
pub const MSG_REQ_REDIS_GEOHASH: msg_type = 152;
pub const MSG_REQ_REDIS_GEODIST: msg_type = 151;
pub const MSG_REQ_REDIS_GEOADD: msg_type = 150;
pub const MSG_REQ_REDIS_ZUNIONSTORE: msg_type = 149;
pub const MSG_REQ_REDIS_ZSCORE: msg_type = 148;
pub const MSG_REQ_REDIS_ZSCAN: msg_type = 147;
pub const MSG_REQ_REDIS_ZUNION: msg_type = 146;
pub const MSG_REQ_REDIS_ZREVRANK: msg_type = 145;
pub const MSG_REQ_REDIS_ZREVRANGEBYSCORE: msg_type = 144;
pub const MSG_REQ_REDIS_ZREVRANGEBYLEX: msg_type = 143;
pub const MSG_REQ_REDIS_ZREVRANGE: msg_type = 142;
pub const MSG_REQ_REDIS_ZREMRANGEBYSCORE: msg_type = 141;
pub const MSG_REQ_REDIS_ZREMRANGEBYLEX: msg_type = 140;
pub const MSG_REQ_REDIS_ZREMRANGEBYRANK: msg_type = 139;
pub const MSG_REQ_REDIS_ZREM: msg_type = 138;
pub const MSG_REQ_REDIS_ZRANK: msg_type = 137;
pub const MSG_REQ_REDIS_ZRANGESTORE: msg_type = 136;
pub const MSG_REQ_REDIS_ZRANGEBYSCORE: msg_type = 135;
pub const MSG_REQ_REDIS_ZRANGEBYLEX: msg_type = 134;
pub const MSG_REQ_REDIS_ZRANGE: msg_type = 133;
pub const MSG_REQ_REDIS_ZRANDMEMBER: msg_type = 132;
pub const MSG_REQ_REDIS_ZPOPMAX: msg_type = 131;
pub const MSG_REQ_REDIS_ZPOPMIN: msg_type = 130;
pub const MSG_REQ_REDIS_ZMSCORE: msg_type = 129;
pub const MSG_REQ_REDIS_ZLEXCOUNT: msg_type = 128;
pub const MSG_REQ_REDIS_ZINTERSTORE: msg_type = 127;
pub const MSG_REQ_REDIS_ZINTER: msg_type = 126;
pub const MSG_REQ_REDIS_ZINCRBY: msg_type = 125;
pub const MSG_REQ_REDIS_ZDIFFSTORE: msg_type = 124;
pub const MSG_REQ_REDIS_ZDIFF: msg_type = 123;
pub const MSG_REQ_REDIS_ZCOUNT: msg_type = 122;
pub const MSG_REQ_REDIS_ZCARD: msg_type = 121;
pub const MSG_REQ_REDIS_ZADD: msg_type = 120;
pub const MSG_REQ_REDIS_SSCAN: msg_type = 119;
pub const MSG_REQ_REDIS_SUNIONSTORE: msg_type = 118;
pub const MSG_REQ_REDIS_SUNION: msg_type = 117;
pub const MSG_REQ_REDIS_SREM: msg_type = 116;
pub const MSG_REQ_REDIS_SRANDMEMBER: msg_type = 115;
pub const MSG_REQ_REDIS_SPOP: msg_type = 114;
pub const MSG_REQ_REDIS_SMOVE: msg_type = 113;
pub const MSG_REQ_REDIS_SMEMBERS: msg_type = 112;
pub const MSG_REQ_REDIS_SMISMEMBER: msg_type = 111;
pub const MSG_REQ_REDIS_SISMEMBER: msg_type = 110;
pub const MSG_REQ_REDIS_SINTERSTORE: msg_type = 109;
pub const MSG_REQ_REDIS_SINTER: msg_type = 108;
pub const MSG_REQ_REDIS_SDIFFSTORE: msg_type = 107;
pub const MSG_REQ_REDIS_SDIFF: msg_type = 106;
pub const MSG_REQ_REDIS_SCARD: msg_type = 105;
pub const MSG_REQ_REDIS_SADD: msg_type = 104;
pub const MSG_REQ_REDIS_RPUSHX: msg_type = 103;
pub const MSG_REQ_REDIS_RPUSH: msg_type = 102;
pub const MSG_REQ_REDIS_RPOPLPUSH: msg_type = 101;
pub const MSG_REQ_REDIS_RPOP: msg_type = 100;
pub const MSG_REQ_REDIS_PFMERGE: msg_type = 99;
pub const MSG_REQ_REDIS_PFCOUNT: msg_type = 98;
pub const MSG_REQ_REDIS_PFADD: msg_type = 97;
pub const MSG_REQ_REDIS_LTRIM: msg_type = 96;
pub const MSG_REQ_REDIS_LSET: msg_type = 95;
pub const MSG_REQ_REDIS_LREM: msg_type = 94;
pub const MSG_REQ_REDIS_LRANGE: msg_type = 93;
pub const MSG_REQ_REDIS_LPUSHX: msg_type = 92;
pub const MSG_REQ_REDIS_LPUSH: msg_type = 91;
pub const MSG_REQ_REDIS_LPOS: msg_type = 90;
pub const MSG_REQ_REDIS_LPOP: msg_type = 89;
pub const MSG_REQ_REDIS_LMOVE: msg_type = 88;
pub const MSG_REQ_REDIS_LLEN: msg_type = 87;
pub const MSG_REQ_REDIS_LINSERT: msg_type = 86;
pub const MSG_REQ_REDIS_LINDEX: msg_type = 85;
pub const MSG_REQ_REDIS_HVALS: msg_type = 84;
pub const MSG_REQ_REDIS_HSTRLEN: msg_type = 83;
pub const MSG_REQ_REDIS_HSCAN: msg_type = 82;
pub const MSG_REQ_REDIS_HSETNX: msg_type = 81;
pub const MSG_REQ_REDIS_HSET: msg_type = 80;
pub const MSG_REQ_REDIS_HRANDFIELD: msg_type = 79;
pub const MSG_REQ_REDIS_HMSET: msg_type = 78;
pub const MSG_REQ_REDIS_HMGET: msg_type = 77;
pub const MSG_REQ_REDIS_HLEN: msg_type = 76;
pub const MSG_REQ_REDIS_HKEYS: msg_type = 75;
pub const MSG_REQ_REDIS_HINCRBYFLOAT: msg_type = 74;
pub const MSG_REQ_REDIS_HINCRBY: msg_type = 73;
pub const MSG_REQ_REDIS_HGETALL: msg_type = 72;
pub const MSG_REQ_REDIS_HGET: msg_type = 71;
pub const MSG_REQ_REDIS_HEXISTS: msg_type = 70;
pub const MSG_REQ_REDIS_HDEL: msg_type = 69;
pub const MSG_REQ_REDIS_STRLEN: msg_type = 68;
pub const MSG_REQ_REDIS_SETRANGE: msg_type = 67;
pub const MSG_REQ_REDIS_SETNX: msg_type = 66;
pub const MSG_REQ_REDIS_SETEX: msg_type = 65;
pub const MSG_REQ_REDIS_SETBIT: msg_type = 64;
pub const MSG_REQ_REDIS_SET: msg_type = 63;
pub const MSG_REQ_REDIS_RESTORE: msg_type = 62;
pub const MSG_REQ_REDIS_PSETEX: msg_type = 61;
pub const MSG_REQ_REDIS_MSET: msg_type = 60;
pub const MSG_REQ_REDIS_MGET: msg_type = 59;
pub const MSG_REQ_REDIS_INCRBYFLOAT: msg_type = 58;
pub const MSG_REQ_REDIS_INCRBY: msg_type = 57;
pub const MSG_REQ_REDIS_INCR: msg_type = 56;
pub const MSG_REQ_REDIS_GETSET: msg_type = 55;
pub const MSG_REQ_REDIS_GETRANGE: msg_type = 54;
pub const MSG_REQ_REDIS_GETEX: msg_type = 53;
pub const MSG_REQ_REDIS_GETDEL: msg_type = 52;
pub const MSG_REQ_REDIS_GETBIT: msg_type = 51;
pub const MSG_REQ_REDIS_GET: msg_type = 50;
pub const MSG_REQ_REDIS_DUMP: msg_type = 49;
pub const MSG_REQ_REDIS_DECRBY: msg_type = 48;
pub const MSG_REQ_REDIS_DECR: msg_type = 47;
pub const MSG_REQ_REDIS_BITPOS: msg_type = 46;
pub const MSG_REQ_REDIS_BITFIELD: msg_type = 45;
pub const MSG_REQ_REDIS_BITCOUNT: msg_type = 44;
pub const MSG_REQ_REDIS_APPEND: msg_type = 43;
pub const MSG_REQ_REDIS_UNLINK: msg_type = 42;
pub const MSG_REQ_REDIS_TYPE: msg_type = 41;
pub const MSG_REQ_REDIS_TTL: msg_type = 40;
pub const MSG_REQ_REDIS_TOUCH: msg_type = 39;
pub const MSG_REQ_REDIS_SORT: msg_type = 38;
pub const MSG_REQ_REDIS_PTTL: msg_type = 37;
pub const MSG_REQ_REDIS_PERSIST: msg_type = 36;
pub const MSG_REQ_REDIS_PEXPIREAT: msg_type = 35;
pub const MSG_REQ_REDIS_PEXPIRE: msg_type = 34;
pub const MSG_REQ_REDIS_MOVE: msg_type = 33;
pub const MSG_REQ_REDIS_EXPIREAT: msg_type = 32;
pub const MSG_REQ_REDIS_EXPIRE: msg_type = 31;
pub const MSG_REQ_REDIS_EXISTS: msg_type = 30;
pub const MSG_REQ_REDIS_DEL: msg_type = 29;
pub const MSG_REQ_REDIS_COPY: msg_type = 28;
pub const MSG_RSP_MC_SERVER_ERROR: msg_type = 27;
pub const MSG_RSP_MC_CLIENT_ERROR: msg_type = 26;
pub const MSG_RSP_MC_ERROR: msg_type = 25;
pub const MSG_RSP_MC_VERSION: msg_type = 24;
pub const MSG_RSP_MC_TOUCHED: msg_type = 23;
pub const MSG_RSP_MC_DELETED: msg_type = 22;
pub const MSG_RSP_MC_VALUE: msg_type = 21;
pub const MSG_RSP_MC_END: msg_type = 20;
pub const MSG_RSP_MC_NOT_FOUND: msg_type = 19;
pub const MSG_RSP_MC_EXISTS: msg_type = 18;
pub const MSG_RSP_MC_NOT_STORED: msg_type = 17;
pub const MSG_RSP_MC_STORED: msg_type = 16;
pub const MSG_RSP_MC_NUM: msg_type = 15;
pub const MSG_REQ_MC_VERSION: msg_type = 14;
pub const MSG_REQ_MC_QUIT: msg_type = 13;
pub const MSG_REQ_MC_TOUCH: msg_type = 12;
pub const MSG_REQ_MC_DECR: msg_type = 11;
pub const MSG_REQ_MC_INCR: msg_type = 10;
pub const MSG_REQ_MC_PREPEND: msg_type = 9;
pub const MSG_REQ_MC_APPEND: msg_type = 8;
pub const MSG_REQ_MC_REPLACE: msg_type = 7;
pub const MSG_REQ_MC_ADD: msg_type = 6;
pub const MSG_REQ_MC_SET: msg_type = 5;
pub const MSG_REQ_MC_CAS: msg_type = 4;
pub const MSG_REQ_MC_DELETE: msg_type = 3;
pub const MSG_REQ_MC_GETS: msg_type = 2;
pub const MSG_REQ_MC_GET: msg_type = 1;
pub const MSG_UNKNOWN: msg_type = 0;
pub type msg_coalesce_t = Option::<unsafe extern "C" fn(*mut msg) -> ()>;
pub type msg_failure_t = Option::<unsafe extern "C" fn(*const msg) -> bool>;
pub type msg_add_auth_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut conn) -> rstatus_t,
>;
pub type msg_reply_t = Option::<unsafe extern "C" fn(*mut msg) -> rstatus_t>;
pub type msg_fragment_t = Option::<
    unsafe extern "C" fn(*mut msg, uint32_t, *mut msg_tqh) -> rstatus_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msg_tqh {
    pub tqh_first: *mut msg,
    pub tqh_last: *mut *mut msg,
}
pub type msg_parse_result_t = msg_parse_result;
pub type msg_parse_result = libc::c_uint;
pub const MSG_PARSE_AGAIN: msg_parse_result = 3;
pub const MSG_PARSE_REPAIR: msg_parse_result = 2;
pub const MSG_PARSE_ERROR: msg_parse_result = 1;
pub const MSG_PARSE_OK: msg_parse_result = 0;
pub type msg_parse_t = Option::<unsafe extern "C" fn(*mut msg) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mhdr {
    pub stqh_first: *mut mbuf,
    pub stqh_last: *mut *mut mbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbuf {
    pub magic: uint32_t,
    pub next: C2RustUnnamed_0,
    pub pos: *mut uint8_t,
    pub last: *mut uint8_t,
    pub start: *mut uint8_t,
    pub end: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub stqe_next: *mut mbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbnode {
    pub left: *mut rbnode,
    pub right: *mut rbnode,
    pub parent: *mut rbnode,
    pub key: int64_t,
    pub data: *mut libc::c_void,
    pub color: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub tqe_next: *mut msg,
    pub tqe_prev: *mut *mut msg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub tqe_next: *mut msg,
    pub tqe_prev: *mut *mut msg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub tqe_next: *mut msg,
    pub tqe_prev: *mut *mut msg,
}
pub type conn_unref_t = Option::<unsafe extern "C" fn(*mut conn) -> ()>;
pub type conn_ref_t = Option::<unsafe extern "C" fn(*mut conn, *mut libc::c_void) -> ()>;
pub type conn_swallow_msg_t = Option::<
    unsafe extern "C" fn(*mut conn, *mut msg, *mut msg) -> (),
>;
pub type conn_post_connect_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut server) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server {
    pub idx: uint32_t,
    pub owner: *mut server_pool,
    pub pname: string,
    pub name: string,
    pub addrstr: string,
    pub port: uint16_t,
    pub weight: uint32_t,
    pub info: sockinfo,
    pub ns_conn_q: uint32_t,
    pub s_conn_q: conn_tqh,
    pub next_retry: int64_t,
    pub failure_count: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conn_tqh {
    pub tqh_first: *mut conn,
    pub tqh_last: *mut *mut conn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockinfo {
    pub family: libc::c_int,
    pub addrlen: socklen_t,
    pub addr: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub in_0: sockaddr_in,
    pub in6: sockaddr_in6,
    pub un: sockaddr_un,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct server_pool {
    pub idx: uint32_t,
    pub ctx: *mut context,
    pub p_conn: *mut conn,
    pub nc_conn_q: uint32_t,
    pub c_conn_q: conn_tqh,
    pub server: array,
    pub ncontinuum: uint32_t,
    pub nserver_continuum: uint32_t,
    pub continuum: *mut continuum,
    pub nlive_server: uint32_t,
    pub next_rebuild: int64_t,
    pub name: string,
    pub addrstr: string,
    pub port: uint16_t,
    pub info: sockinfo,
    pub perm: mode_t,
    pub dist_type: libc::c_int,
    pub key_hash_type: libc::c_int,
    pub key_hash: hash_t,
    pub hash_tag: string,
    pub timeout: libc::c_int,
    pub backlog: libc::c_int,
    pub redis_db: libc::c_int,
    pub client_connections: uint32_t,
    pub server_connections: uint32_t,
    pub server_retry_timeout: int64_t,
    pub server_failure_limit: uint32_t,
    pub redis_auth: string,
    pub require_auth: libc::c_uint,
    #[bitfield(name = "auto_eject_hosts", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "preconnect", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "redis", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "tcpkeepalive", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "reuseport", ty = "libc::c_uint", bits = "4..=4")]
    pub auto_eject_hosts_preconnect_redis_tcpkeepalive_reuseport: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type hash_t = Option::<
    unsafe extern "C" fn(*const libc::c_char, size_t) -> uint32_t,
>;
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct continuum {
    pub index: uint32_t,
    pub value: uint32_t,
}
pub type conn_active_t = Option::<unsafe extern "C" fn(*const conn) -> bool>;
pub type conn_close_t = Option::<unsafe extern "C" fn(*mut context, *mut conn) -> ()>;
pub type conn_send_done_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut msg) -> (),
>;
pub type conn_send_next_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn) -> *mut msg,
>;
pub type conn_send_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn) -> rstatus_t,
>;
pub type conn_recv_done_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut msg, *mut msg) -> (),
>;
pub type conn_recv_next_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, bool) -> *mut msg,
>;
pub type conn_recv_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn) -> rstatus_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub tqe_next: *mut conn,
    pub tqe_prev: *mut *mut conn,
}
pub type C2RustUnnamed_7 = libc::c_uint;
pub const SW_SENTINEL: C2RustUnnamed_7 = 22;
pub const SW_ALMOST_DONE: C2RustUnnamed_7 = 21;
pub const SW_AFTER_NOREPLY: C2RustUnnamed_7 = 20;
pub const SW_NOREPLY: C2RustUnnamed_7 = 19;
pub const SW_CRLF: C2RustUnnamed_7 = 18;
pub const SW_RUNTO_CRLF: C2RustUnnamed_7 = 17;
pub const SW_NUM: C2RustUnnamed_7 = 16;
pub const SW_SPACES_BEFORE_NUM: C2RustUnnamed_7 = 15;
pub const SW_VAL: C2RustUnnamed_7 = 14;
pub const SW_RUNTO_VAL: C2RustUnnamed_7 = 13;
pub const SW_CAS: C2RustUnnamed_7 = 12;
pub const SW_SPACES_BEFORE_CAS: C2RustUnnamed_7 = 11;
pub const SW_VLEN: C2RustUnnamed_7 = 10;
pub const SW_SPACES_BEFORE_VLEN: C2RustUnnamed_7 = 9;
pub const SW_EXPIRY: C2RustUnnamed_7 = 8;
pub const SW_SPACES_BEFORE_EXPIRY: C2RustUnnamed_7 = 7;
pub const SW_FLAGS: C2RustUnnamed_7 = 6;
pub const SW_SPACES_BEFORE_FLAGS: C2RustUnnamed_7 = 5;
pub const SW_SPACES_BEFORE_KEYS: C2RustUnnamed_7 = 4;
pub const SW_KEY: C2RustUnnamed_7 = 3;
pub const SW_SPACES_BEFORE_KEY: C2RustUnnamed_7 = 2;
pub const SW_REQ_TYPE: C2RustUnnamed_7 = 1;
pub const SW_START: C2RustUnnamed_7 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keypos {
    pub start: *mut uint8_t,
    pub end: *mut uint8_t,
}
pub type C2RustUnnamed_8 = libc::c_uint;
pub const SW_SENTINEL_0: C2RustUnnamed_8 = 16;
pub const SW_ALMOST_DONE_0: C2RustUnnamed_8 = 15;
pub const SW_CRLF_0: C2RustUnnamed_8 = 14;
pub const SW_RUNTO_CRLF_0: C2RustUnnamed_8 = 13;
pub const SW_END: C2RustUnnamed_8 = 12;
pub const SW_VAL_LF: C2RustUnnamed_8 = 11;
pub const SW_VAL_0: C2RustUnnamed_8 = 10;
pub const SW_RUNTO_VAL_0: C2RustUnnamed_8 = 9;
pub const SW_VLEN_0: C2RustUnnamed_8 = 8;
pub const SW_SPACES_BEFORE_VLEN_0: C2RustUnnamed_8 = 7;
pub const SW_FLAGS_0: C2RustUnnamed_8 = 6;
pub const SW_SPACES_BEFORE_FLAGS_0: C2RustUnnamed_8 = 5;
pub const SW_KEY_0: C2RustUnnamed_8 = 4;
pub const SW_SPACES_BEFORE_KEY_0: C2RustUnnamed_8 = 3;
pub const SW_RSP_STR: C2RustUnnamed_8 = 2;
pub const SW_RSP_NUM: C2RustUnnamed_8 = 1;
pub const SW_START_0: C2RustUnnamed_8 = 0;
#[inline]
unsafe extern "C" fn array_n(mut a: *const array) -> uint32_t {
    return (*a).nelem;
}
#[inline]
unsafe extern "C" fn mbuf_empty(mut mbuf: *const mbuf) -> bool {
    return (*mbuf).pos == (*mbuf).last;
}
unsafe extern "C" fn memcache_storage(mut r: *const msg) -> bool {
    match (*r).type_0 as libc::c_uint {
        5 | 4 | 6 | 7 | 8 | 9 => return 1 as libc::c_int != 0,
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn memcache_cas(mut r: *const msg) -> bool {
    if (*r).type_0 as libc::c_uint == MSG_REQ_MC_CAS as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn memcache_retrieval(mut r: *const msg) -> bool {
    match (*r).type_0 as libc::c_uint {
        1 | 2 => return 1 as libc::c_int != 0,
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn memcache_should_fragment(mut r: *const msg) -> bool {
    match (*r).type_0 as libc::c_uint {
        1 | 2 => return array_n((*r).keys) != 1 as libc::c_int as libc::c_uint,
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn memcache_arithmetic(mut r: *const msg) -> bool {
    match (*r).type_0 as libc::c_uint {
        10 | 11 => return 1 as libc::c_int != 0,
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn memcache_delete(mut r: *const msg) -> bool {
    if (*r).type_0 as libc::c_uint == MSG_REQ_MC_DELETE as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn memcache_touch(mut r: *const msg) -> bool {
    if (*r).type_0 as libc::c_uint == MSG_REQ_MC_TOUCH as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn memcache_parse_req(mut r: *mut msg) {
    let mut current_block: u64;
    let mut b: *mut mbuf = 0 as *mut mbuf;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut m: *mut uint8_t = 0 as *mut uint8_t;
    let mut ch: uint8_t = 0;
    let mut state: C2RustUnnamed_7 = SW_START;
    state = (*r).state as C2RustUnnamed_7;
    b = if ((*r).mhdr.stqh_first).is_null() {
        0 as *mut mbuf
    } else {
        ((*r).mhdr.stqh_last as *mut libc::c_char)
            .offset(
                -(&mut (*(0 as *mut libc::c_void as *mut mbuf)).next
                    as *mut C2RustUnnamed_0 as size_t as isize),
            ) as *mut libc::c_void as *mut mbuf
    };
    p = (*r).pos;
    loop {
        if !(p < (*b).last) {
            current_block = 1254794021369287194;
            break;
        }
        ch = *p;
        match state as libc::c_uint {
            0 => {
                if !(ch as libc::c_int == ' ' as i32) {
                    if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                        as libc::c_int
                        & _ISlower as libc::c_int as libc::c_ushort as libc::c_int == 0
                    {
                        current_block = 3369507108967252529;
                        break;
                    }
                    (*r).token = p;
                    state = SW_REQ_TYPE;
                }
            }
            1 => {
                if ch as libc::c_int == ' ' as i32
                    || ch as libc::c_int == 13 as libc::c_int as uint8_t as libc::c_int
                {
                    m = (*r).token;
                    (*r).token = 0 as *mut uint8_t;
                    (*r).type_0 = MSG_UNKNOWN;
                    (*r).narg = ((*r).narg).wrapping_add(1);
                    (*r).narg;
                    match p.offset_from(m) as libc::c_long {
                        3 => {
                            if *(m as *mut uint32_t)
                                == ((' ' as i32) << 24 as libc::c_int
                                    | ('t' as i32) << 16 as libc::c_int
                                    | ('e' as i32) << 8 as libc::c_int | 'g' as i32)
                                    as libc::c_uint
                            {
                                (*r).type_0 = MSG_REQ_MC_GET;
                            } else if *(m as *mut uint32_t)
                                == ((' ' as i32) << 24 as libc::c_int
                                    | ('t' as i32) << 16 as libc::c_int
                                    | ('e' as i32) << 8 as libc::c_int | 's' as i32)
                                    as libc::c_uint
                            {
                                (*r).type_0 = MSG_REQ_MC_SET;
                            } else if *(m as *mut uint32_t)
                                == ((' ' as i32) << 24 as libc::c_int
                                    | ('d' as i32) << 16 as libc::c_int
                                    | ('d' as i32) << 8 as libc::c_int | 'a' as i32)
                                    as libc::c_uint
                            {
                                (*r).type_0 = MSG_REQ_MC_ADD;
                            } else if *(m as *mut uint32_t)
                                == ((' ' as i32) << 24 as libc::c_int
                                    | ('s' as i32) << 16 as libc::c_int
                                    | ('a' as i32) << 8 as libc::c_int | 'c' as i32)
                                    as libc::c_uint
                            {
                                (*r).type_0 = MSG_REQ_MC_CAS;
                            }
                        }
                        4 => {
                            if *(m as *mut uint32_t)
                                == (('s' as i32) << 24 as libc::c_int
                                    | ('t' as i32) << 16 as libc::c_int
                                    | ('e' as i32) << 8 as libc::c_int | 'g' as i32)
                                    as libc::c_uint
                            {
                                (*r).type_0 = MSG_REQ_MC_GETS;
                            } else if *(m as *mut uint32_t)
                                == (('r' as i32) << 24 as libc::c_int
                                    | ('c' as i32) << 16 as libc::c_int
                                    | ('n' as i32) << 8 as libc::c_int | 'i' as i32)
                                    as libc::c_uint
                            {
                                (*r).type_0 = MSG_REQ_MC_INCR;
                            } else if *(m as *mut uint32_t)
                                == (('r' as i32) << 24 as libc::c_int
                                    | ('c' as i32) << 16 as libc::c_int
                                    | ('e' as i32) << 8 as libc::c_int | 'd' as i32)
                                    as libc::c_uint
                            {
                                (*r).type_0 = MSG_REQ_MC_DECR;
                            } else if *(m as *mut uint32_t)
                                == (('t' as i32) << 24 as libc::c_int
                                    | ('i' as i32) << 16 as libc::c_int
                                    | ('u' as i32) << 8 as libc::c_int | 'q' as i32)
                                    as libc::c_uint
                            {
                                (*r).type_0 = MSG_REQ_MC_QUIT;
                                (*r).set_quit(1 as libc::c_int as libc::c_uint);
                            }
                        }
                        5 => {
                            if *(m as *mut uint32_t)
                                == (('c' as i32) << 24 as libc::c_int
                                    | ('u' as i32) << 16 as libc::c_int
                                    | ('o' as i32) << 8 as libc::c_int | 't' as i32)
                                    as libc::c_uint
                                && *m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32
                            {
                                (*r).type_0 = MSG_REQ_MC_TOUCH;
                            }
                        }
                        6 => {
                            if *(m as *mut uint32_t)
                                == (('e' as i32) << 24 as libc::c_int
                                    | ('p' as i32) << 16 as libc::c_int
                                    | ('p' as i32) << 8 as libc::c_int | 'a' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    & 0xffff as libc::c_int as libc::c_uint
                                    == (('d' as i32) << 8 as libc::c_int | 'n' as i32)
                                        as libc::c_uint
                            {
                                (*r).type_0 = MSG_REQ_MC_APPEND;
                            } else if *(m as *mut uint32_t)
                                == (('e' as i32) << 24 as libc::c_int
                                    | ('l' as i32) << 16 as libc::c_int
                                    | ('e' as i32) << 8 as libc::c_int | 'd' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    & 0xffff as libc::c_int as libc::c_uint
                                    == (('e' as i32) << 8 as libc::c_int | 't' as i32)
                                        as libc::c_uint
                            {
                                (*r).type_0 = MSG_REQ_MC_DELETE;
                            }
                        }
                        7 => {
                            if *(m as *mut uint32_t)
                                == (('p' as i32) << 24 as libc::c_int
                                    | ('e' as i32) << 16 as libc::c_int
                                    | ('r' as i32) << 8 as libc::c_int | 'p' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    & 0xffff as libc::c_int as libc::c_uint
                                    == (('n' as i32) << 8 as libc::c_int | 'e' as i32)
                                        as libc::c_uint
                                && *m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                            {
                                (*r).type_0 = MSG_REQ_MC_PREPEND;
                            } else if *(m as *mut uint32_t)
                                == (('l' as i32) << 24 as libc::c_int
                                    | ('p' as i32) << 16 as libc::c_int
                                    | ('e' as i32) << 8 as libc::c_int | 'r' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    & 0xffff as libc::c_int as libc::c_uint
                                    == (('c' as i32) << 8 as libc::c_int | 'a' as i32)
                                        as libc::c_uint
                                && *m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                            {
                                (*r).type_0 = MSG_REQ_MC_REPLACE;
                            } else if *(m as *mut uint32_t)
                                == (('s' as i32) << 24 as libc::c_int
                                    | ('r' as i32) << 16 as libc::c_int
                                    | ('e' as i32) << 8 as libc::c_int | 'v' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    & 0xffff as libc::c_int as libc::c_uint
                                    == (('o' as i32) << 8 as libc::c_int | 'i' as i32)
                                        as libc::c_uint
                                && *m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                            {
                                (*r).type_0 = MSG_REQ_MC_VERSION;
                                if !msg_set_placeholder_key(r) {
                                    current_block = 18093980355565901102;
                                    break;
                                }
                            }
                        }
                        _ => {}
                    }
                    match (*r).type_0 as libc::c_uint {
                        1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 => {
                            current_block = 14823214990557839738;
                            match current_block {
                                1086029098901714860 => {
                                    p = p.offset(-(1 as libc::c_int as isize));
                                    state = SW_CRLF;
                                }
                                _ => {
                                    if ch as libc::c_int
                                        == 13 as libc::c_int as uint8_t as libc::c_int
                                    {
                                        current_block = 3369507108967252529;
                                        break;
                                    }
                                    state = SW_SPACES_BEFORE_KEY;
                                }
                            }
                        }
                        14 | 13 => {
                            current_block = 1086029098901714860;
                            match current_block {
                                1086029098901714860 => {
                                    p = p.offset(-(1 as libc::c_int as isize));
                                    state = SW_CRLF;
                                }
                                _ => {
                                    if ch as libc::c_int
                                        == 13 as libc::c_int as uint8_t as libc::c_int
                                    {
                                        current_block = 3369507108967252529;
                                        break;
                                    }
                                    state = SW_SPACES_BEFORE_KEY;
                                }
                            }
                        }
                        0 => {
                            current_block = 3369507108967252529;
                            break;
                        }
                        _ => {}
                    }
                } else if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                    as libc::c_int
                    & _ISlower as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    current_block = 3369507108967252529;
                    break;
                }
            }
            2 => {
                if ch as libc::c_int != ' ' as i32 {
                    p = p.offset(-(1 as libc::c_int as isize));
                    (*r).token = 0 as *mut uint8_t;
                    state = SW_KEY;
                }
            }
            3 => {
                if ((*r).token).is_null() {
                    (*r).token = p;
                }
                if ch as libc::c_int == ' ' as i32
                    || ch as libc::c_int == 13 as libc::c_int as uint8_t as libc::c_int
                {
                    let mut kpos: *mut keypos = 0 as *mut keypos;
                    let mut keylen: libc::c_int = p.offset_from((*r).token)
                        as libc::c_long as libc::c_int;
                    if keylen > 250 as libc::c_int {
                        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
                            _log(
                                b"nc_memcache.c\0" as *const u8 as *const libc::c_char,
                                391 as libc::c_int,
                                0 as libc::c_int,
                                b"parsed bad req %lu of type %d with key prefix '%.*s...' and length %d that exceeds maximum key length\0"
                                    as *const u8 as *const libc::c_char,
                                (*r).id,
                                (*r).type_0 as libc::c_uint,
                                16 as libc::c_int,
                                (*r).token,
                                p.offset_from((*r).token) as libc::c_long as libc::c_int,
                            );
                        }
                        current_block = 3369507108967252529;
                        break;
                    } else if keylen == 0 as libc::c_int {
                        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
                            _log(
                                b"nc_memcache.c\0" as *const u8 as *const libc::c_char,
                                395 as libc::c_int,
                                0 as libc::c_int,
                                b"parsed bad req %lu of type %d with an empty key\0"
                                    as *const u8 as *const libc::c_char,
                                (*r).id,
                                (*r).type_0 as libc::c_uint,
                            );
                        }
                        current_block = 3369507108967252529;
                        break;
                    } else {
                        kpos = array_push((*r).keys) as *mut keypos;
                        if kpos.is_null() {
                            current_block = 18093980355565901102;
                            break;
                        }
                        (*kpos).start = (*r).token;
                        (*kpos).end = p;
                        (*r).narg = ((*r).narg).wrapping_add(1);
                        (*r).narg;
                        (*r).token = 0 as *mut uint8_t;
                        if memcache_storage(r) {
                            state = SW_SPACES_BEFORE_FLAGS;
                        } else if memcache_arithmetic(r) as libc::c_int != 0
                            || memcache_touch(r) as libc::c_int != 0
                        {
                            state = SW_SPACES_BEFORE_NUM;
                        } else if memcache_retrieval(r) {
                            state = SW_SPACES_BEFORE_KEYS;
                        } else {
                            state = SW_RUNTO_CRLF;
                        }
                        if ch as libc::c_int
                            == 13 as libc::c_int as uint8_t as libc::c_int
                        {
                            if memcache_storage(r) as libc::c_int != 0
                                || memcache_arithmetic(r) as libc::c_int != 0
                            {
                                current_block = 3369507108967252529;
                                break;
                            }
                            p = p.offset(-(1 as libc::c_int as isize));
                        }
                    }
                }
            }
            4 => {
                match ch as libc::c_int {
                    32 => {}
                    13 => {
                        state = SW_ALMOST_DONE;
                    }
                    _ => {
                        (*r).token = 0 as *mut uint8_t;
                        p = p.offset(-(1 as libc::c_int as isize));
                        state = SW_KEY;
                    }
                }
            }
            5 => {
                if ch as libc::c_int != ' ' as i32 {
                    if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                    {
                        current_block = 3369507108967252529;
                        break;
                    }
                    (*r).token = p;
                    state = SW_FLAGS;
                }
            }
            6 => {
                if !(*(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
                {
                    if !(ch as libc::c_int == ' ' as i32) {
                        current_block = 3369507108967252529;
                        break;
                    }
                    (*r).token = 0 as *mut uint8_t;
                    state = SW_SPACES_BEFORE_EXPIRY;
                }
            }
            7 => {
                if ch as libc::c_int != ' ' as i32 {
                    if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                    {
                        current_block = 3369507108967252529;
                        break;
                    }
                    (*r).token = p;
                    state = SW_EXPIRY;
                }
            }
            8 => {
                if !(*(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
                {
                    if !(ch as libc::c_int == ' ' as i32) {
                        current_block = 3369507108967252529;
                        break;
                    }
                    (*r).token = 0 as *mut uint8_t;
                    state = SW_SPACES_BEFORE_VLEN;
                }
            }
            9 => {
                if ch as libc::c_int != ' ' as i32 {
                    if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                    {
                        current_block = 3369507108967252529;
                        break;
                    }
                    (*r).vlen = (ch as libc::c_int - '0' as i32) as uint32_t;
                    state = SW_VLEN;
                }
            }
            10 => {
                if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    (*r)
                        .vlen = ((*r).vlen)
                        .wrapping_mul(10 as libc::c_int as libc::c_uint)
                        .wrapping_add((ch as libc::c_int - '0' as i32) as uint32_t);
                } else if memcache_cas(r) {
                    if ch as libc::c_int != ' ' as i32 {
                        current_block = 3369507108967252529;
                        break;
                    }
                    p = p.offset(-(1 as libc::c_int as isize));
                    (*r).token = 0 as *mut uint8_t;
                    state = SW_SPACES_BEFORE_CAS;
                } else {
                    if !(ch as libc::c_int == ' ' as i32
                        || ch as libc::c_int
                            == 13 as libc::c_int as uint8_t as libc::c_int)
                    {
                        current_block = 3369507108967252529;
                        break;
                    }
                    p = p.offset(-(1 as libc::c_int as isize));
                    (*r).token = 0 as *mut uint8_t;
                    state = SW_RUNTO_CRLF;
                }
            }
            11 => {
                if ch as libc::c_int != ' ' as i32 {
                    if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                    {
                        current_block = 3369507108967252529;
                        break;
                    }
                    (*r).token = p;
                    state = SW_CAS;
                }
            }
            12 => {
                if !(*(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
                {
                    if !(ch as libc::c_int == ' ' as i32
                        || ch as libc::c_int
                            == 13 as libc::c_int as uint8_t as libc::c_int)
                    {
                        current_block = 3369507108967252529;
                        break;
                    }
                    p = p.offset(-(1 as libc::c_int as isize));
                    (*r).token = 0 as *mut uint8_t;
                    state = SW_RUNTO_CRLF;
                }
            }
            13 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 3369507108967252529;
                        break;
                    }
                }
                state = SW_VAL;
            }
            14 => {
                m = p.offset((*r).vlen as isize);
                if m >= (*b).last {
                    (*r)
                        .vlen = ((*r).vlen as libc::c_uint)
                        .wrapping_sub(
                            ((*b).last).offset_from(p) as libc::c_long as uint32_t,
                        ) as uint32_t as uint32_t;
                    m = ((*b).last).offset(-(1 as libc::c_int as isize));
                    p = m;
                } else {
                    match *m as libc::c_int {
                        13 => {}
                        _ => {
                            current_block = 3369507108967252529;
                            break;
                        }
                    }
                    p = m;
                    state = SW_ALMOST_DONE;
                }
            }
            15 => {
                if ch as libc::c_int != ' ' as i32 {
                    if !(*(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                        || ch as libc::c_int == '-' as i32)
                    {
                        current_block = 3369507108967252529;
                        break;
                    }
                    (*r).token = p;
                    state = SW_NUM;
                }
            }
            16 => {
                if !(*(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
                {
                    if !(ch as libc::c_int == ' ' as i32
                        || ch as libc::c_int
                            == 13 as libc::c_int as uint8_t as libc::c_int)
                    {
                        current_block = 3369507108967252529;
                        break;
                    }
                    (*r).token = 0 as *mut uint8_t;
                    p = p.offset(-(1 as libc::c_int as isize));
                    state = SW_RUNTO_CRLF;
                }
            }
            17 => {
                match ch as libc::c_int {
                    32 => {}
                    110 => {
                        current_block = 4257177969859078588;
                        match current_block {
                            18280587616738688663 => {
                                if memcache_storage(r) {
                                    state = SW_RUNTO_VAL;
                                } else {
                                    state = SW_ALMOST_DONE;
                                }
                            }
                            _ => {
                                if !(memcache_storage(r) as libc::c_int != 0
                                    || memcache_arithmetic(r) as libc::c_int != 0
                                    || memcache_delete(r) as libc::c_int != 0
                                    || memcache_touch(r) as libc::c_int != 0)
                                {
                                    current_block = 3369507108967252529;
                                    break;
                                }
                                (*r).token = p;
                                state = SW_NOREPLY;
                            }
                        }
                    }
                    13 => {
                        current_block = 18280587616738688663;
                        match current_block {
                            18280587616738688663 => {
                                if memcache_storage(r) {
                                    state = SW_RUNTO_VAL;
                                } else {
                                    state = SW_ALMOST_DONE;
                                }
                            }
                            _ => {
                                if !(memcache_storage(r) as libc::c_int != 0
                                    || memcache_arithmetic(r) as libc::c_int != 0
                                    || memcache_delete(r) as libc::c_int != 0
                                    || memcache_touch(r) as libc::c_int != 0)
                                {
                                    current_block = 3369507108967252529;
                                    break;
                                }
                                (*r).token = p;
                                state = SW_NOREPLY;
                            }
                        }
                    }
                    _ => {
                        current_block = 3369507108967252529;
                        break;
                    }
                }
            }
            19 => {
                match ch as libc::c_int {
                    32 | 13 => {
                        m = (*r).token;
                        if !(p.offset_from(m) as libc::c_long
                            == 7 as libc::c_int as libc::c_long
                            && (*(m as *mut uint32_t)
                                == (('e' as i32) << 24 as libc::c_int
                                    | ('r' as i32) << 16 as libc::c_int
                                    | ('o' as i32) << 8 as libc::c_int | 'n' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    & 0xffff as libc::c_int as libc::c_uint
                                    == (('l' as i32) << 8 as libc::c_int | 'p' as i32)
                                        as libc::c_uint
                                && *m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'y' as i32))
                        {
                            current_block = 3369507108967252529;
                            break;
                        }
                        (*r).token = 0 as *mut uint8_t;
                        (*r).set_noreply(1 as libc::c_int as libc::c_uint);
                        state = SW_AFTER_NOREPLY;
                        p = p.offset(-(1 as libc::c_int as isize));
                    }
                    _ => {}
                }
            }
            20 => {
                match ch as libc::c_int {
                    32 => {}
                    13 => {
                        if memcache_storage(r) {
                            state = SW_RUNTO_VAL;
                        } else {
                            state = SW_ALMOST_DONE;
                        }
                    }
                    _ => {
                        current_block = 3369507108967252529;
                        break;
                    }
                }
            }
            18 => {
                match ch as libc::c_int {
                    32 => {}
                    13 => {
                        state = SW_ALMOST_DONE;
                    }
                    _ => {
                        current_block = 3369507108967252529;
                        break;
                    }
                }
            }
            21 => {
                match ch as libc::c_int {
                    10 => {
                        current_block = 2942740671786505091;
                        break;
                    }
                    _ => {
                        current_block = 3369507108967252529;
                        break;
                    }
                }
            }
            22 | _ => {}
        }
        p = p.offset(1);
        p;
    }
    match current_block {
        3369507108967252529 => {
            (*r).result = MSG_PARSE_ERROR;
            (*r).state = state as libc::c_int;
            *__errno_location() = 22 as libc::c_int;
            return;
        }
        1254794021369287194 => {
            (*r).pos = p;
            (*r).state = state as libc::c_int;
            if (*b).last == (*b).end && !((*r).token).is_null() {
                (*r).pos = (*r).token;
                (*r).token = 0 as *mut uint8_t;
                (*r).result = MSG_PARSE_REPAIR;
            } else {
                (*r).result = MSG_PARSE_AGAIN;
            }
            return;
        }
        18093980355565901102 => {
            (*r).result = MSG_PARSE_ERROR;
            (*r).state = state as libc::c_int;
            return;
        }
        _ => {
            (*r).pos = p.offset(1 as libc::c_int as isize);
            (*r).state = SW_START as libc::c_int;
            (*r).result = MSG_PARSE_OK;
            return;
        }
    };
}
pub unsafe extern "C" fn memcache_parse_rsp(mut r: *mut msg) {
    let mut current_block: u64;
    let mut b: *mut mbuf = 0 as *mut mbuf;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut m: *mut uint8_t = 0 as *mut uint8_t;
    let mut ch: uint8_t = 0;
    let mut state: C2RustUnnamed_8 = SW_START_0;
    state = (*r).state as C2RustUnnamed_8;
    b = if ((*r).mhdr.stqh_first).is_null() {
        0 as *mut mbuf
    } else {
        ((*r).mhdr.stqh_last as *mut libc::c_char)
            .offset(
                -(&mut (*(0 as *mut libc::c_void as *mut mbuf)).next
                    as *mut C2RustUnnamed_0 as size_t as isize),
            ) as *mut libc::c_void as *mut mbuf
    };
    p = (*r).pos;
    loop {
        if !(p < (*b).last) {
            current_block = 18323319510560553714;
            break;
        }
        ch = *p;
        match state as libc::c_uint {
            0 => {
                if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    state = SW_RSP_NUM;
                } else {
                    state = SW_RSP_STR;
                }
                p = p.offset(-(1 as libc::c_int as isize));
            }
            1 => {
                if ((*r).token).is_null() {
                    (*r).token = p;
                }
                if !(*(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
                {
                    if !(ch as libc::c_int == ' ' as i32
                        || ch as libc::c_int
                            == 13 as libc::c_int as uint8_t as libc::c_int)
                    {
                        current_block = 8323215225707587601;
                        break;
                    }
                    (*r).token = 0 as *mut uint8_t;
                    (*r).type_0 = MSG_RSP_MC_NUM;
                    p = p.offset(-(1 as libc::c_int as isize));
                    state = SW_CRLF_0;
                }
            }
            2 => {
                if ((*r).token).is_null() {
                    (*r).token = p;
                }
                if ch as libc::c_int == ' ' as i32
                    || ch as libc::c_int == 13 as libc::c_int as uint8_t as libc::c_int
                {
                    m = (*r).token;
                    (*r).type_0 = MSG_UNKNOWN;
                    match p.offset_from(m) as libc::c_long {
                        3 => {
                            if *(m as *mut uint32_t)
                                == (('\r' as i32) << 24 as libc::c_int
                                    | ('D' as i32) << 16 as libc::c_int
                                    | ('N' as i32) << 8 as libc::c_int | 'E' as i32)
                                    as libc::c_uint
                            {
                                (*r).type_0 = MSG_RSP_MC_END;
                                (*r).end = m;
                            }
                        }
                        5 => {
                            if *(m as *mut uint32_t)
                                == (('U' as i32) << 24 as libc::c_int
                                    | ('L' as i32) << 16 as libc::c_int
                                    | ('A' as i32) << 8 as libc::c_int | 'V' as i32)
                                    as libc::c_uint
                                && *m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'E' as i32
                            {
                                (*r).type_0 = MSG_RSP_MC_VALUE;
                            } else if *(m as *mut uint32_t)
                                == (('O' as i32) << 24 as libc::c_int
                                    | ('R' as i32) << 16 as libc::c_int
                                    | ('R' as i32) << 8 as libc::c_int | 'E' as i32)
                                    as libc::c_uint
                                && *m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'R' as i32
                            {
                                (*r).type_0 = MSG_RSP_MC_ERROR;
                            }
                        }
                        6 => {
                            if *(m as *mut uint32_t)
                                == (('R' as i32) << 24 as libc::c_int
                                    | ('O' as i32) << 16 as libc::c_int
                                    | ('T' as i32) << 8 as libc::c_int | 'S' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    & 0xffff as libc::c_int as libc::c_uint
                                    == (('D' as i32) << 8 as libc::c_int | 'E' as i32)
                                        as libc::c_uint
                            {
                                (*r).type_0 = MSG_RSP_MC_STORED;
                            } else if *(m as *mut uint32_t)
                                == (('S' as i32) << 24 as libc::c_int
                                    | ('I' as i32) << 16 as libc::c_int
                                    | ('X' as i32) << 8 as libc::c_int | 'E' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    & 0xffff as libc::c_int as libc::c_uint
                                    == (('S' as i32) << 8 as libc::c_int | 'T' as i32)
                                        as libc::c_uint
                            {
                                (*r).type_0 = MSG_RSP_MC_EXISTS;
                            }
                        }
                        7 => {
                            if *(m as *mut uint32_t)
                                == (('E' as i32) << 24 as libc::c_int
                                    | ('L' as i32) << 16 as libc::c_int
                                    | ('E' as i32) << 8 as libc::c_int | 'D' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    & 0xffff as libc::c_int as libc::c_uint
                                    == (('E' as i32) << 8 as libc::c_int | 'T' as i32)
                                        as libc::c_uint
                                && *m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'D' as i32
                            {
                                (*r).type_0 = MSG_RSP_MC_DELETED;
                            } else if *(m as *mut uint32_t)
                                == (('C' as i32) << 24 as libc::c_int
                                    | ('U' as i32) << 16 as libc::c_int
                                    | ('O' as i32) << 8 as libc::c_int | 'T' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    & 0xffff as libc::c_int as libc::c_uint
                                    == (('E' as i32) << 8 as libc::c_int | 'H' as i32)
                                        as libc::c_uint
                                && *m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'D' as i32
                            {
                                (*r).type_0 = MSG_RSP_MC_TOUCHED;
                            } else if *(m as *mut uint32_t)
                                == (('S' as i32) << 24 as libc::c_int
                                    | ('R' as i32) << 16 as libc::c_int
                                    | ('E' as i32) << 8 as libc::c_int | 'V' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    & 0xffff as libc::c_int as libc::c_uint
                                    == (('O' as i32) << 8 as libc::c_int | 'I' as i32)
                                        as libc::c_uint
                                && *m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'N' as i32
                            {
                                (*r).type_0 = MSG_RSP_MC_VERSION;
                            }
                        }
                        9 => {
                            if *(m as *mut uint32_t)
                                == (('_' as i32) << 24 as libc::c_int
                                    | ('T' as i32) << 16 as libc::c_int
                                    | ('O' as i32) << 8 as libc::c_int | 'N' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    == (('N' as i32) << 24 as libc::c_int
                                        | ('U' as i32) << 16 as libc::c_int
                                        | ('O' as i32) << 8 as libc::c_int | 'F' as i32)
                                        as libc::c_uint
                                && *m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'D' as i32
                            {
                                (*r).type_0 = MSG_RSP_MC_NOT_FOUND;
                            }
                        }
                        10 => {
                            if *(m as *mut uint32_t)
                                == (('_' as i32) << 24 as libc::c_int
                                    | ('T' as i32) << 16 as libc::c_int
                                    | ('O' as i32) << 8 as libc::c_int | 'N' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    == (('R' as i32) << 24 as libc::c_int
                                        | ('O' as i32) << 16 as libc::c_int
                                        | ('T' as i32) << 8 as libc::c_int | 'S' as i32)
                                        as libc::c_uint
                                && *(m as *mut uint32_t).offset(2 as libc::c_int as isize)
                                    & 0xffff as libc::c_int as libc::c_uint
                                    == (('D' as i32) << 8 as libc::c_int | 'E' as i32)
                                        as libc::c_uint
                            {
                                (*r).type_0 = MSG_RSP_MC_NOT_STORED;
                            }
                        }
                        12 => {
                            if *(m as *mut uint32_t)
                                == (('E' as i32) << 24 as libc::c_int
                                    | ('I' as i32) << 16 as libc::c_int
                                    | ('L' as i32) << 8 as libc::c_int | 'C' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    == (('E' as i32) << 24 as libc::c_int
                                        | ('_' as i32) << 16 as libc::c_int
                                        | ('T' as i32) << 8 as libc::c_int | 'N' as i32)
                                        as libc::c_uint
                                && *(m as *mut uint32_t).offset(2 as libc::c_int as isize)
                                    == (('R' as i32) << 24 as libc::c_int
                                        | ('O' as i32) << 16 as libc::c_int
                                        | ('R' as i32) << 8 as libc::c_int | 'R' as i32)
                                        as libc::c_uint
                            {
                                (*r).type_0 = MSG_RSP_MC_CLIENT_ERROR;
                            } else if *(m as *mut uint32_t)
                                == (('V' as i32) << 24 as libc::c_int
                                    | ('R' as i32) << 16 as libc::c_int
                                    | ('E' as i32) << 8 as libc::c_int | 'S' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    == (('E' as i32) << 24 as libc::c_int
                                        | ('_' as i32) << 16 as libc::c_int
                                        | ('R' as i32) << 8 as libc::c_int | 'E' as i32)
                                        as libc::c_uint
                                && *(m as *mut uint32_t).offset(2 as libc::c_int as isize)
                                    == (('R' as i32) << 24 as libc::c_int
                                        | ('O' as i32) << 16 as libc::c_int
                                        | ('R' as i32) << 8 as libc::c_int | 'R' as i32)
                                        as libc::c_uint
                            {
                                (*r).type_0 = MSG_RSP_MC_SERVER_ERROR;
                            }
                        }
                        _ => {}
                    }
                    match (*r).type_0 as libc::c_uint {
                        0 => {
                            current_block = 8323215225707587601;
                            break;
                        }
                        16 | 17 | 18 | 19 | 22 | 23 => {
                            state = SW_CRLF_0;
                        }
                        20 => {
                            state = SW_CRLF_0;
                        }
                        21 => {
                            state = SW_SPACES_BEFORE_KEY_0;
                        }
                        25 => {
                            state = SW_CRLF_0;
                        }
                        26 | 27 | 24 => {
                            state = SW_RUNTO_CRLF_0;
                        }
                        _ => {}
                    }
                    p = p.offset(-(1 as libc::c_int as isize));
                }
            }
            3 => {
                if ch as libc::c_int != ' ' as i32 {
                    state = SW_KEY_0;
                    p = p.offset(-(1 as libc::c_int as isize));
                }
            }
            4 => {
                if ch as libc::c_int == ' ' as i32 {
                    state = SW_SPACES_BEFORE_FLAGS_0;
                }
            }
            5 => {
                if ch as libc::c_int != ' ' as i32 {
                    if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                    {
                        current_block = 8323215225707587601;
                        break;
                    }
                    state = SW_FLAGS_0;
                    p = p.offset(-(1 as libc::c_int as isize));
                }
            }
            6 => {
                ((*r).token).is_null();
                if !(*(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
                {
                    if !(ch as libc::c_int == ' ' as i32) {
                        current_block = 8323215225707587601;
                        break;
                    }
                    state = SW_SPACES_BEFORE_VLEN_0;
                }
            }
            7 => {
                if ch as libc::c_int != ' ' as i32 {
                    if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                    {
                        current_block = 8323215225707587601;
                        break;
                    }
                    p = p.offset(-(1 as libc::c_int as isize));
                    state = SW_VLEN_0;
                    (*r).vlen = 0 as libc::c_int as uint32_t;
                }
            }
            8 => {
                if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    (*r)
                        .vlen = ((*r).vlen)
                        .wrapping_mul(10 as libc::c_int as libc::c_uint)
                        .wrapping_add((ch as libc::c_int - '0' as i32) as uint32_t);
                } else {
                    if !(ch as libc::c_int == ' ' as i32
                        || ch as libc::c_int
                            == 13 as libc::c_int as uint8_t as libc::c_int)
                    {
                        current_block = 8323215225707587601;
                        break;
                    }
                    p = p.offset(-(1 as libc::c_int as isize));
                    state = SW_RUNTO_CRLF_0;
                }
            }
            9 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 8323215225707587601;
                        break;
                    }
                }
                state = SW_VAL_0;
                (*r).token = 0 as *mut uint8_t;
            }
            10 => {
                m = p.offset((*r).vlen as isize);
                if m >= (*b).last {
                    (*r)
                        .vlen = ((*r).vlen as libc::c_uint)
                        .wrapping_sub(
                            ((*b).last).offset_from(p) as libc::c_long as uint32_t,
                        ) as uint32_t as uint32_t;
                    m = ((*b).last).offset(-(1 as libc::c_int as isize));
                    p = m;
                } else {
                    match *m as libc::c_int {
                        13 => {}
                        _ => {
                            current_block = 8323215225707587601;
                            break;
                        }
                    }
                    p = m;
                    state = SW_VAL_LF;
                }
            }
            11 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 8323215225707587601;
                        break;
                    }
                }
                state = SW_RSP_STR;
            }
            12 => {
                if ((*r).token).is_null() {
                    if ch as libc::c_int != 'E' as i32 {
                        current_block = 8323215225707587601;
                        break;
                    }
                    (*r).token = p;
                } else if ch as libc::c_int
                    == 13 as libc::c_int as uint8_t as libc::c_int
                {
                    m = (*r).token;
                    (*r).token = 0 as *mut uint8_t;
                    match p.offset_from(m) as libc::c_long {
                        3 => {}
                        _ => {
                            current_block = 8323215225707587601;
                            break;
                        }
                    }
                    if *(m as *mut uint32_t)
                        == (('\r' as i32) << 24 as libc::c_int
                            | ('D' as i32) << 16 as libc::c_int
                            | ('N' as i32) << 8 as libc::c_int | 'E' as i32)
                            as libc::c_uint
                    {
                        (*r).end = m;
                        state = SW_ALMOST_DONE_0;
                    }
                }
            }
            13 => {
                match ch as libc::c_int {
                    13 => {
                        if (*r).type_0 as libc::c_uint
                            == MSG_RSP_MC_VALUE as libc::c_int as libc::c_uint
                        {
                            state = SW_RUNTO_VAL_0;
                        } else {
                            state = SW_ALMOST_DONE_0;
                        }
                    }
                    _ => {}
                }
            }
            14 => {
                match ch as libc::c_int {
                    32 => {}
                    13 => {
                        state = SW_ALMOST_DONE_0;
                    }
                    _ => {
                        current_block = 8323215225707587601;
                        break;
                    }
                }
            }
            15 => {
                match ch as libc::c_int {
                    10 => {
                        current_block = 3297745280902459415;
                        break;
                    }
                    _ => {
                        current_block = 8323215225707587601;
                        break;
                    }
                }
            }
            16 | _ => {}
        }
        p = p.offset(1);
        p;
    }
    match current_block {
        8323215225707587601 => {
            (*r).result = MSG_PARSE_ERROR;
            (*r).state = state as libc::c_int;
            *__errno_location() = 22 as libc::c_int;
            return;
        }
        18323319510560553714 => {
            (*r).pos = p;
            (*r).state = state as libc::c_int;
            if (*b).last == (*b).end && !((*r).token).is_null() {
                if state as libc::c_uint <= SW_RUNTO_VAL_0 as libc::c_int as libc::c_uint
                    || state as libc::c_uint == SW_CRLF_0 as libc::c_int as libc::c_uint
                    || state as libc::c_uint
                        == SW_ALMOST_DONE_0 as libc::c_int as libc::c_uint
                {
                    (*r).state = SW_START_0 as libc::c_int;
                }
                (*r).pos = (*r).token;
                (*r).token = 0 as *mut uint8_t;
                (*r).result = MSG_PARSE_REPAIR;
            } else {
                (*r).result = MSG_PARSE_AGAIN;
            }
            return;
        }
        _ => {
            (*r).pos = p.offset(1 as libc::c_int as isize);
            (*r).state = SW_START_0 as libc::c_int;
            (*r).token = 0 as *mut uint8_t;
            (*r).result = MSG_PARSE_OK;
            return;
        }
    };
}
pub unsafe extern "C" fn memcache_failure(mut r: *const msg) -> bool {
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn memcache_append_key(
    mut r: *mut msg,
    mut key: *const uint8_t,
    mut keylen: uint32_t,
) -> rstatus_t {
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    let mut kpos: *mut keypos = 0 as *mut keypos;
    mbuf = msg_ensure_mbuf(
        r,
        keylen.wrapping_add(2 as libc::c_int as libc::c_uint) as size_t,
    );
    if mbuf.is_null() {
        return -(3 as libc::c_int);
    }
    kpos = array_push((*r).keys) as *mut keypos;
    if kpos.is_null() {
        return -(3 as libc::c_int);
    }
    (*kpos).start = (*mbuf).last;
    (*kpos).end = ((*mbuf).last).offset(keylen as isize);
    mbuf_copy(mbuf, key, keylen as size_t);
    (*r).mlen = ((*r).mlen as libc::c_uint).wrapping_add(keylen) as uint32_t as uint32_t;
    mbuf_copy(
        mbuf,
        b" \0" as *const u8 as *const libc::c_char as *const uint8_t,
        1 as libc::c_int as size_t,
    );
    (*r)
        .mlen = ((*r).mlen as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn memcache_fragment_retrieval(
    mut r: *mut msg,
    mut nserver: uint32_t,
    mut frag_msgq: *mut msg_tqh,
    mut key_step: uint32_t,
) -> rstatus_t {
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    let mut sub_msgs: *mut *mut msg = 0 as *mut *mut msg;
    let mut i: uint32_t = 0;
    let mut status: rstatus_t = 0;
    sub_msgs = _nc_zalloc(
        (nserver as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut msg>() as libc::c_ulong),
        b"nc_memcache.c\0" as *const u8 as *const libc::c_char,
        1293 as libc::c_int,
    ) as *mut *mut msg;
    if sub_msgs.is_null() {
        return -(3 as libc::c_int);
    }
    (*r)
        .frag_seq = _nc_alloc(
        (array_n((*r).keys) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut msg>() as libc::c_ulong),
        b"nc_memcache.c\0" as *const u8 as *const libc::c_char,
        1299 as libc::c_int,
    ) as *mut *mut msg;
    if ((*r).frag_seq).is_null() {
        _nc_free(
            sub_msgs as *mut libc::c_void,
            b"nc_memcache.c\0" as *const u8 as *const libc::c_char,
            1301 as libc::c_int,
        );
        sub_msgs = 0 as *mut *mut msg;
        return -(3 as libc::c_int);
    }
    mbuf = (*r).mhdr.stqh_first;
    (*mbuf).pos = (*mbuf).start;
    while *(*mbuf).pos as libc::c_int != ' ' as i32 {
        (*mbuf).pos = ((*mbuf).pos).offset(1);
        (*mbuf).pos;
    }
    (*mbuf).pos = ((*mbuf).pos).offset(1);
    (*mbuf).pos;
    (*r).frag_id = msg_gen_frag_id();
    (*r).nfrag = 0 as libc::c_int as uint32_t;
    (*r).frag_owner = r;
    i = 0 as libc::c_int as uint32_t;
    while i < array_n((*r).keys) {
        let mut sub_msg: *mut msg = 0 as *mut msg;
        let mut kpos: *mut keypos = array_get((*r).keys, i) as *mut keypos;
        let mut idx: uint32_t = msg_backend_idx(
            r,
            (*kpos).start,
            ((*kpos).end).offset_from((*kpos).start) as libc::c_long as uint32_t,
        );
        if (*sub_msgs.offset(idx as isize)).is_null() {
            let ref mut fresh0 = *sub_msgs.offset(idx as isize);
            *fresh0 = msg_get((*r).owner, (*r).request() != 0, (*r).redis() != 0);
            if (*sub_msgs.offset(idx as isize)).is_null() {
                _nc_free(
                    sub_msgs as *mut libc::c_void,
                    b"nc_memcache.c\0" as *const u8 as *const libc::c_char,
                    1333 as libc::c_int,
                );
                sub_msgs = 0 as *mut *mut msg;
                return -(3 as libc::c_int);
            }
        }
        sub_msg = *sub_msgs.offset(idx as isize);
        let ref mut fresh1 = *((*r).frag_seq).offset(i as isize);
        *fresh1 = sub_msg;
        (*sub_msg).narg = ((*sub_msg).narg).wrapping_add(1);
        (*sub_msg).narg;
        status = memcache_append_key(
            sub_msg,
            (*kpos).start,
            ((*kpos).end).offset_from((*kpos).start) as libc::c_long as uint32_t,
        );
        if status != 0 as libc::c_int {
            _nc_free(
                sub_msgs as *mut libc::c_void,
                b"nc_memcache.c\0" as *const u8 as *const libc::c_char,
                1342 as libc::c_int,
            );
            sub_msgs = 0 as *mut *mut msg;
            return status;
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as uint32_t;
    while i < nserver {
        let mut sub_msg_0: *mut msg = *sub_msgs.offset(i as isize);
        if !sub_msg_0.is_null() {
            if (*r).type_0 as libc::c_uint
                == MSG_REQ_MC_GET as libc::c_int as libc::c_uint
            {
                status = msg_prepend(
                    sub_msg_0,
                    b"get \0" as *const u8 as *const libc::c_char as *const uint8_t,
                    4 as libc::c_int as size_t,
                );
            } else if (*r).type_0 as libc::c_uint
                == MSG_REQ_MC_GETS as libc::c_int as libc::c_uint
            {
                status = msg_prepend(
                    sub_msg_0,
                    b"gets \0" as *const u8 as *const libc::c_char as *const uint8_t,
                    5 as libc::c_int as size_t,
                );
            }
            if status != 0 as libc::c_int {
                _nc_free(
                    sub_msgs as *mut libc::c_void,
                    b"nc_memcache.c\0" as *const u8 as *const libc::c_char,
                    1363 as libc::c_int,
                );
                sub_msgs = 0 as *mut *mut msg;
                return status;
            }
            status = msg_append(
                sub_msg_0,
                b"\r\n\0" as *const u8 as *const libc::c_char as *const uint8_t,
                (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            if status != 0 as libc::c_int {
                _nc_free(
                    sub_msgs as *mut libc::c_void,
                    b"nc_memcache.c\0" as *const u8 as *const libc::c_char,
                    1370 as libc::c_int,
                );
                sub_msgs = 0 as *mut *mut msg;
                return status;
            }
            (*sub_msg_0).type_0 = (*r).type_0;
            (*sub_msg_0).frag_id = (*r).frag_id;
            (*sub_msg_0).frag_owner = (*r).frag_owner;
            (*sub_msg_0).m_tqe.tqe_next = 0 as *mut msg;
            (*sub_msg_0).m_tqe.tqe_prev = (*frag_msgq).tqh_last;
            *(*frag_msgq).tqh_last = sub_msg_0;
            (*frag_msgq).tqh_last = &mut (*sub_msg_0).m_tqe.tqe_next;
            (*r).nfrag = ((*r).nfrag).wrapping_add(1);
            (*r).nfrag;
        }
        i = i.wrapping_add(1);
        i;
    }
    _nc_free(
        sub_msgs as *mut libc::c_void,
        b"nc_memcache.c\0" as *const u8 as *const libc::c_char,
        1382 as libc::c_int,
    );
    sub_msgs = 0 as *mut *mut msg;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn memcache_fragment(
    mut r: *mut msg,
    mut nserver: uint32_t,
    mut frag_msgq: *mut msg_tqh,
) -> rstatus_t {
    if memcache_should_fragment(r) {
        return memcache_fragment_retrieval(
            r,
            nserver,
            frag_msgq,
            1 as libc::c_int as uint32_t,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn memcache_pre_coalesce(mut r: *mut msg) {
    let mut pr: *mut msg = (*r).peer;
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    if (*pr).frag_id == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    (*(*pr).frag_owner).nfrag_done = ((*(*pr).frag_owner).nfrag_done).wrapping_add(1);
    (*(*pr).frag_owner).nfrag_done;
    match (*r).type_0 as libc::c_uint {
        21 | 20 => {
            loop {
                mbuf = if ((*r).mhdr.stqh_first).is_null() {
                    0 as *mut mbuf
                } else {
                    ((*r).mhdr.stqh_last as *mut libc::c_char)
                        .offset(
                            -(&mut (*(0 as *mut libc::c_void as *mut mbuf)).next
                                as *mut C2RustUnnamed_0 as size_t as isize),
                        ) as *mut libc::c_void as *mut mbuf
                };
                if (*r).end >= (*mbuf).pos && (*r).end < (*mbuf).last {
                    (*r)
                        .mlen = ((*r).mlen as libc::c_uint)
                        .wrapping_sub(
                            ((*mbuf).last).offset_from((*r).end) as libc::c_long
                                as uint32_t,
                        ) as uint32_t as uint32_t;
                    (*mbuf).last = (*r).end;
                    break;
                } else {
                    (*r)
                        .mlen = ((*r).mlen as libc::c_uint)
                        .wrapping_sub(mbuf_length(mbuf)) as uint32_t as uint32_t;
                    mbuf_remove(&mut (*r).mhdr, mbuf);
                    mbuf_put(mbuf);
                }
            }
        }
        _ => {
            mbuf = (*r).mhdr.stqh_first;
            (*pr).set_error(1 as libc::c_int as libc::c_uint);
            (*pr).err = 22 as libc::c_int;
        }
    };
}
unsafe extern "C" fn memcache_copy_bulk(
    mut dst: *mut msg,
    mut src: *mut msg,
) -> rstatus_t {
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    let mut nbuf: *mut mbuf = 0 as *mut mbuf;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut last: *const uint8_t = 0 as *const uint8_t;
    let mut len: uint32_t = 0 as libc::c_int as uint32_t;
    let mut bytes: uint32_t = 0 as libc::c_int as uint32_t;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    mbuf = (*src).mhdr.stqh_first;
    while !mbuf.is_null() && mbuf_empty(mbuf) as libc::c_int != 0 {
        mbuf_remove(&mut (*src).mhdr, mbuf);
        mbuf_put(mbuf);
        mbuf = (*src).mhdr.stqh_first;
    }
    mbuf = (*src).mhdr.stqh_first;
    if mbuf.is_null() {
        return 0 as libc::c_int;
    }
    p = (*mbuf).pos;
    last = (*mbuf).last;
    i = 0 as libc::c_int as uint32_t;
    while p < last {
        if *p as libc::c_int == ' ' as i32 {
            i = i.wrapping_add(1);
            i;
            if i >= 3 as libc::c_int as libc::c_uint {
                p = p.offset(1);
                p;
                break;
            }
        }
        p = p.offset(1);
        p;
    }
    len = 0 as libc::c_int as uint32_t;
    while p < last
        && *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        len = len
            .wrapping_mul(10 as libc::c_int as libc::c_uint)
            .wrapping_add((*p as libc::c_int - '0' as i32) as uint32_t);
        p = p.offset(1);
        p;
    }
    while p < last && '\r' as i32 != *p as libc::c_int {
        p = p.offset(1);
        p;
    }
    len = (len as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ) as uint32_t as uint32_t;
    len = (len as libc::c_long + p.offset_from((*mbuf).pos) as libc::c_long) as uint32_t;
    if p >= last {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_memcache.c\0" as *const u8 as *const libc::c_char,
                1529 as libc::c_int,
                0 as libc::c_int,
                b"Saw memcache value response where header was not parsed or header length %d unexpectedly exceeded mbuf size limit\0"
                    as *const u8 as *const libc::c_char,
                p.offset_from((*mbuf).pos) as libc::c_long as libc::c_int,
            );
        }
        return -(1 as libc::c_int);
    }
    bytes = len;
    while !mbuf.is_null() {
        if mbuf_length(mbuf) <= len {
            nbuf = (*mbuf).next.stqe_next;
            mbuf_remove(&mut (*src).mhdr, mbuf);
            mbuf_insert(&mut (*dst).mhdr, mbuf);
            len = (len as libc::c_uint).wrapping_sub(mbuf_length(mbuf)) as uint32_t
                as uint32_t;
            mbuf = nbuf;
        } else {
            nbuf = mbuf_get();
            if nbuf.is_null() {
                return -(3 as libc::c_int);
            }
            mbuf_copy(nbuf, (*mbuf).pos, len as size_t);
            mbuf_insert(&mut (*dst).mhdr, nbuf);
            (*mbuf).pos = ((*mbuf).pos).offset(len as isize);
            break;
        }
    }
    (*dst)
        .mlen = ((*dst).mlen as libc::c_uint).wrapping_add(bytes) as uint32_t
        as uint32_t;
    (*src)
        .mlen = ((*src).mlen as libc::c_uint).wrapping_sub(bytes) as uint32_t
        as uint32_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn memcache_post_coalesce(mut request: *mut msg) {
    let mut response: *mut msg = (*request).peer;
    let mut sub_msg: *mut msg = 0 as *mut msg;
    let mut i: uint32_t = 0;
    let mut status: rstatus_t = 0;
    if (*request).error() as libc::c_int != 0 || (*request).ferror() as libc::c_int != 0
    {
        (*(*response).owner).err = 1 as libc::c_int;
        return;
    }
    i = 0 as libc::c_int as uint32_t;
    while i < array_n((*request).keys) {
        sub_msg = (**((*request).frag_seq).offset(i as isize)).peer;
        if sub_msg.is_null() {
            (*(*response).owner).err = 1 as libc::c_int;
            return;
        }
        status = memcache_copy_bulk(response, sub_msg);
        if status != 0 as libc::c_int {
            (*(*response).owner).err = 1 as libc::c_int;
            return;
        }
        i = i.wrapping_add(1);
        i;
    }
    status = msg_append(
        response,
        b"END\r\n\0" as *const u8 as *const libc::c_char as *const uint8_t,
        5 as libc::c_int as size_t,
    );
    if status != 0 as libc::c_int {
        (*(*response).owner).err = 1 as libc::c_int;
        return;
    }
}
pub unsafe extern "C" fn memcache_post_connect(
    mut ctx: *mut context,
    mut conn: *mut conn,
    mut server: *mut server,
) {}
pub unsafe extern "C" fn memcache_swallow_msg(
    mut conn: *mut conn,
    mut pmsg: *mut msg,
    mut msg: *mut msg,
) {}
pub unsafe extern "C" fn memcache_add_auth(
    mut ctx: *mut context,
    mut c_conn: *mut conn,
    mut s_conn: *mut conn,
) -> rstatus_t {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn memcache_reply(mut r: *mut msg) -> rstatus_t {
    return 0 as libc::c_int;
}
