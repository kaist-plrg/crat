use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type epoll_event;
    pub type conf;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn log10(_: libc::c_double) -> libc::c_double;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
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
    fn mbuf_put(mbuf: *mut mbuf);
    fn mbuf_rewind(mbuf: *mut mbuf);
    fn mbuf_length(mbuf: *const mbuf) -> uint32_t;
    fn mbuf_data_size() -> size_t;
    fn mbuf_insert(mhdr: *mut mhdr, mbuf: *mut mbuf);
    fn mbuf_remove(mhdr: *mut mhdr, mbuf: *mut mbuf);
    fn mbuf_copy(mbuf: *mut mbuf, pos: *const uint8_t, n: size_t);
    fn msg_set_placeholder_key(r: *mut msg) -> bool;
    fn msg_append(msg: *mut msg, pos: *const uint8_t, n: size_t) -> rstatus_t;
    fn msg_prepend_format(msg: *mut msg, fmt: *const libc::c_char, _: ...) -> rstatus_t;
    fn msg_put(msg: *mut msg);
    fn msg_get(conn: *mut conn, request: bool, redis: bool) -> *mut msg;
    fn msg_ensure_mbuf(msg: *mut msg, len: size_t) -> *mut mbuf;
    fn msg_backend_idx(
        msg: *const msg,
        key: *const uint8_t,
        keylen: uint32_t,
    ) -> uint32_t;
    fn msg_gen_frag_id() -> uint64_t;
    fn conn_authenticated(conn: *const conn) -> bool;
    fn msg_send(ctx: *mut context, conn: *mut conn) -> rstatus_t;
    fn req_server_enqueue_imsgq_head(ctx: *mut context, conn: *mut conn, msg: *mut msg);
}
pub type size_t = libc::c_ulong;
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
pub const SW_SENTINEL: C2RustUnnamed_7 = 27;
pub const SW_ARGN_LF: C2RustUnnamed_7 = 26;
pub const SW_ARGN: C2RustUnnamed_7 = 25;
pub const SW_ARGN_LEN_LF: C2RustUnnamed_7 = 24;
pub const SW_ARGN_LEN: C2RustUnnamed_7 = 23;
pub const SW_ARG3_LF: C2RustUnnamed_7 = 22;
pub const SW_ARG3: C2RustUnnamed_7 = 21;
pub const SW_ARG3_LEN_LF: C2RustUnnamed_7 = 20;
pub const SW_ARG3_LEN: C2RustUnnamed_7 = 19;
pub const SW_ARG2_LF: C2RustUnnamed_7 = 18;
pub const SW_ARG2: C2RustUnnamed_7 = 17;
pub const SW_ARG2_LEN_LF: C2RustUnnamed_7 = 16;
pub const SW_ARG2_LEN: C2RustUnnamed_7 = 15;
pub const SW_ARG1_LF: C2RustUnnamed_7 = 14;
pub const SW_ARG1: C2RustUnnamed_7 = 13;
pub const SW_ARG1_LEN_LF: C2RustUnnamed_7 = 12;
pub const SW_ARG1_LEN: C2RustUnnamed_7 = 11;
pub const SW_KEY_LF: C2RustUnnamed_7 = 10;
pub const SW_KEY: C2RustUnnamed_7 = 9;
pub const SW_KEY_LEN_LF: C2RustUnnamed_7 = 8;
pub const SW_KEY_LEN: C2RustUnnamed_7 = 7;
pub const SW_REQ_TYPE_LF: C2RustUnnamed_7 = 6;
pub const SW_REQ_TYPE: C2RustUnnamed_7 = 5;
pub const SW_REQ_TYPE_LEN_LF: C2RustUnnamed_7 = 4;
pub const SW_REQ_TYPE_LEN: C2RustUnnamed_7 = 3;
pub const SW_NARG_LF: C2RustUnnamed_7 = 2;
pub const SW_NARG: C2RustUnnamed_7 = 1;
pub const SW_START: C2RustUnnamed_7 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keypos {
    pub start: *mut uint8_t,
    pub end: *mut uint8_t,
}
pub type C2RustUnnamed_8 = libc::c_uint;
pub const SW_SENTINEL_0: C2RustUnnamed_8 = 17;
pub const SW_ALMOST_DONE: C2RustUnnamed_8 = 16;
pub const SW_RUNTO_CRLF: C2RustUnnamed_8 = 15;
pub const SW_MULTIBULK_ARGN_LF: C2RustUnnamed_8 = 14;
pub const SW_MULTIBULK_ARGN: C2RustUnnamed_8 = 13;
pub const SW_MULTIBULK_ARGN_LEN_LF: C2RustUnnamed_8 = 12;
pub const SW_MULTIBULK_ARGN_LEN: C2RustUnnamed_8 = 11;
pub const SW_MULTIBULK_NARG_LF: C2RustUnnamed_8 = 10;
pub const SW_MULTIBULK: C2RustUnnamed_8 = 9;
pub const SW_BULK_ARG_LF: C2RustUnnamed_8 = 8;
pub const SW_BULK_ARG: C2RustUnnamed_8 = 7;
pub const SW_BULK_LF: C2RustUnnamed_8 = 6;
pub const SW_BULK: C2RustUnnamed_8 = 5;
pub const SW_SIMPLE: C2RustUnnamed_8 = 4;
pub const SW_INTEGER_START: C2RustUnnamed_8 = 3;
pub const SW_ERROR: C2RustUnnamed_8 = 2;
pub const SW_STATUS: C2RustUnnamed_8 = 1;
pub const SW_START_0: C2RustUnnamed_8 = 0;
#[inline]
unsafe extern "C" fn array_n(mut a: *const array) -> uint32_t {
    return (*a).nelem;
}
#[inline]
unsafe extern "C" fn mbuf_empty(mut mbuf: *const mbuf) -> bool {
    return (*mbuf).pos == (*mbuf).last;
}
static mut rsp_no_password: string = string {
    len: 0,
    data: 0 as *const uint8_t as *mut uint8_t,
};
static mut rsp_invalid_password: string = string {
    len: 0,
    data: 0 as *const uint8_t as *mut uint8_t,
};
static mut rsp_auth_required: string = string {
    len: 0,
    data: 0 as *const uint8_t as *mut uint8_t,
};
static mut rsp_pong: string = string {
    len: 0,
    data: 0 as *const uint8_t as *mut uint8_t,
};
static mut rsp_ok: string = string {
    len: 0,
    data: 0 as *const uint8_t as *mut uint8_t,
};
unsafe extern "C" fn redis_argz(mut r: *const msg) -> bool {
    match (*r).type_0 as libc::c_uint {
        160 | 161 | 164 => return 1 as libc::c_int != 0,
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn redis_arg0(mut r: *const msg) -> bool {
    match (*r).type_0 as libc::c_uint {
        36 | 37 | 40 | 41 | 49 | 47 | 50 | 52 | 56 | 68 | 72 | 75 | 76 | 84 | 87 | 105
        | 112 | 121 | 162 => return 1 as libc::c_int != 0,
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn redis_arg1(mut r: *const msg) -> bool {
    match (*r).type_0 as libc::c_uint {
        31 | 32 | 34 | 35 | 33 | 43 | 48 | 51 | 55 | 57 | 58 | 66 | 70 | 71 | 83 | 85
        | 101 | 110 | 137 | 145 | 148 => return 1 as libc::c_int != 0,
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn redis_arg2(mut r: *const msg) -> bool {
    match (*r).type_0 as libc::c_uint {
        54 | 61 | 64 | 65 | 67 | 73 | 74 | 81 | 93 | 94 | 95 | 96 | 113 | 122 | 128 | 125
        | 140 | 139 | 141 => return 1 as libc::c_int != 0,
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn redis_arg3(mut r: *const msg) -> bool {
    match (*r).type_0 as libc::c_uint {
        86 | 88 => return 1 as libc::c_int != 0,
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn redis_argn(mut r: *const msg) -> bool {
    match (*r).type_0 as libc::c_uint {
        38 | 28 | 44 | 46 | 45 | 30 | 53 | 63 | 69 | 77 | 78 | 82 | 80 | 79 | 91 | 92
        | 102 | 103 | 89 | 100 | 90 | 104 | 106 | 107 | 108 | 109 | 116 | 117 | 118 | 115
        | 119 | 114 | 111 | 97 | 99 | 98 | 120 | 123 | 124 | 126 | 127 | 129 | 131 | 130
        | 132 | 133 | 134 | 135 | 136 | 138 | 142 | 143 | 144 | 147 | 146 | 149 | 151
        | 155 | 152 | 150 | 153 | 154 | 156 | 157 | 62 => return 1 as libc::c_int != 0,
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn redis_argx(mut r: *const msg) -> bool {
    match (*r).type_0 as libc::c_uint {
        59 | 29 | 42 | 39 => return 1 as libc::c_int != 0,
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn redis_argkvx(mut r: *const msg) -> bool {
    match (*r).type_0 as libc::c_uint {
        60 => return 1 as libc::c_int != 0,
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn redis_argeval(mut r: *const msg) -> bool {
    match (*r).type_0 as libc::c_uint {
        158 | 159 => return 1 as libc::c_int != 0,
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn redis_nokey(mut r: *const msg) -> bool {
    match (*r).type_0 as libc::c_uint {
        165 => return 1 as libc::c_int != 0,
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn redis_error(mut r: *const msg) -> bool {
    match (*r).type_0 as libc::c_uint {
        167 | 168 | 169 | 170 | 171 | 172 | 173 | 174 | 175 | 176 | 177 | 178 | 179
        | 180 => return 1 as libc::c_int != 0,
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn redis_parse_req(mut r: *mut msg) {
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
    's_34: loop {
        if !(p < (*b).last) {
            current_block = 4873392082507075976;
            break;
        }
        ch = *p;
        match state as libc::c_uint {
            0 => {
                if ch as libc::c_int != '*' as i32 {
                    current_block = 9430895183509304024;
                    break;
                } else {
                    (*r).token = p;
                    (*r).narg_start = p;
                    (*r).rnarg = 0 as libc::c_int as uint32_t;
                    state = SW_NARG;
                }
            }
            1 => {
                if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    (*r)
                        .rnarg = ((*r).rnarg)
                        .wrapping_mul(10 as libc::c_int as libc::c_uint)
                        .wrapping_add((ch as libc::c_int - '0' as i32) as uint32_t);
                } else {
                    if !(ch as libc::c_int
                        == 13 as libc::c_int as uint8_t as libc::c_int)
                    {
                        current_block = 9430895183509304024;
                        break;
                    }
                    if (*r).rnarg == 0 as libc::c_int as libc::c_uint {
                        current_block = 9430895183509304024;
                        break;
                    }
                    (*r).narg = (*r).rnarg;
                    (*r).narg_end = p;
                    (*r).token = 0 as *mut uint8_t;
                    state = SW_NARG_LF;
                }
            }
            2 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 9430895183509304024;
                        break;
                    }
                }
                state = SW_REQ_TYPE_LEN;
            }
            3 => {
                if ((*r).token).is_null() {
                    if ch as libc::c_int != '$' as i32 {
                        current_block = 9430895183509304024;
                        break;
                    }
                    (*r).token = p;
                    (*r).rlen = 0 as libc::c_int as uint32_t;
                } else if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    (*r)
                        .rlen = ((*r).rlen)
                        .wrapping_mul(10 as libc::c_int as libc::c_uint)
                        .wrapping_add((ch as libc::c_int - '0' as i32) as uint32_t);
                } else {
                    if !(ch as libc::c_int
                        == 13 as libc::c_int as uint8_t as libc::c_int)
                    {
                        current_block = 9430895183509304024;
                        break;
                    }
                    if (*r).rlen == 0 as libc::c_int as libc::c_uint
                        || (*r).rnarg == 0 as libc::c_int as libc::c_uint
                    {
                        current_block = 9430895183509304024;
                        break;
                    }
                    (*r).rnarg = ((*r).rnarg).wrapping_sub(1);
                    (*r).rnarg;
                    (*r).token = 0 as *mut uint8_t;
                    state = SW_REQ_TYPE_LEN_LF;
                }
            }
            4 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 9430895183509304024;
                        break;
                    }
                }
                state = SW_REQ_TYPE;
            }
            5 => {
                if ((*r).token).is_null() {
                    (*r).token = p;
                }
                m = ((*r).token).offset((*r).rlen as isize);
                if m >= (*b).last {
                    m = ((*b).last).offset(-(1 as libc::c_int as isize));
                    p = m;
                } else {
                    if *m as libc::c_int != 13 as libc::c_int as uint8_t as libc::c_int {
                        current_block = 9430895183509304024;
                        break;
                    }
                    p = m;
                    (*r).rlen = 0 as libc::c_int as uint32_t;
                    m = (*r).token;
                    (*r).token = 0 as *mut uint8_t;
                    (*r).type_0 = MSG_UNKNOWN;
                    match p.offset_from(m) as libc::c_long {
                        3 => {
                            if (*m.offset(0 as libc::c_int as isize) as libc::c_int
                                == 'g' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_GET;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SET;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 't' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_TTL;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'd' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_DEL;
                            }
                        }
                        4 => {
                            if (*m.offset(0 as libc::c_int as isize) as libc::c_int
                                == 'p' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_PTTL;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'd' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_DECR;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'd' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_DUMP;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'h' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_HDEL;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'h' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_HGET;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'h' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_HLEN;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'h' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_HSET;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'i' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_INCR;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'l' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_LLEN;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'l' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_LPOP;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'l' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_LPOS;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'l' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_LREM;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'l' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_LSET;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'r' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_RPOP;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SADD;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SPOP;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SREM;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 't' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'y' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'y' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_TYPE;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'm' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_MGET;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'm' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_MSET;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZADD;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZREM;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'e' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'v' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'v' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_EVAL;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SORT;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'p' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'g' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_PING;
                                (*r).set_noforward(1 as libc::c_int as libc::c_uint);
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'q' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'q' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_QUIT;
                                (*r).set_quit(1 as libc::c_int as libc::c_uint);
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'a' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'h' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_AUTH;
                                (*r).set_noforward(1 as libc::c_int as libc::c_uint);
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'm' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'v' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'v' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_MOVE;
                                (*r).set_noforward(1 as libc::c_int as libc::c_uint);
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'c' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'y' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'y' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_COPY;
                            }
                        }
                        5 => {
                            if (*m.offset(0 as libc::c_int as isize) as libc::c_int
                                == 'h' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'k' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'k' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'y' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'y' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_HKEYS;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'h' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_HMGET;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'h' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_HMSET;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'h' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'v' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'v' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_HVALS;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'h' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_HSCAN;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'l' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'h' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_LPUSH;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'l' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_LTRIM;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'r' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'h' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_RPUSH;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SCARD;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'f' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'f' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'f' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'f' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SDIFF;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'x' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'x' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SETEX;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'x' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'x' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SETNX;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'v' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'v' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SMOVE;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SSCAN;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZCARD;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'f' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'f' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'f' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'f' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZDIFF;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'k' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'k' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZRANK;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZSCAN;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'p' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'f' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'f' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_PFADD;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'g' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'x' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'x' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_GETEX;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 't' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'h' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_TOUCH;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'l' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'v' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'v' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_LMOVE;
                            }
                        }
                        6 => {
                            if (*m.offset(0 as libc::c_int as isize) as libc::c_int
                                == 'a' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_APPEND;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'b' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_BITPOS;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'd' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'y' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'y' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_DECRBY;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'e' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'x' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'x' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_EXISTS;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'e' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'x' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'x' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_EXPIRE;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'g' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_GETBIT;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'g' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_GETSET;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'p' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'x' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'x' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_PSETEX;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'h' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'x' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'x' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_HSETNX;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'i' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'y' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'y' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_INCRBY;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'l' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'x' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'x' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_LINDEX;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'l' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'x' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'x' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_LPUSHX;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'l' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_LRANGE;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'r' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'x' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'x' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_RPUSHX;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SETBIT;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SINTER;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_STRLEN;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SUNION;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZCOUNT;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZRANGE;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZSCORE;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'g' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_GEOPOS;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'g' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_GEOADD;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'g' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_GETDEL;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZUNION;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZINTER;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'u' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'k' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'k' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_UNLINK;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'l' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'w' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'w' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_LOLWUT;
                                if !msg_set_placeholder_key(r) {
                                    current_block = 581218188993692030;
                                    break;
                                }
                            }
                        }
                        7 => {
                            if (*m.offset(0 as libc::c_int as isize) as libc::c_int
                                == 'p' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_PERSIST;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'p' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'x' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'x' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_PEXPIRE;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'h' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'x' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'x' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_HEXISTS;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'h' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_HGETALL;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'h' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'y' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'y' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_HINCRBY;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'l' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_LINSERT;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'y' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'y' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZINCRBY;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'e' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'v' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'v' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_EVALSHA;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'r' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_RESTORE;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'p' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'f' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'f' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_PFCOUNT;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'p' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'f' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'f' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_PFMERGE;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZMSCORE;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZPOPMIN;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'x' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'x' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZPOPMAX;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'g' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_GEODIST;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'g' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'h' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_GEOHASH;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'h' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_HSTRLEN;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'c' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_COMMAND;
                                if !msg_set_placeholder_key(r) {
                                    current_block = 581218188993692030;
                                    break;
                                }
                            }
                        }
                        8 => {
                            if (*m.offset(0 as libc::c_int as isize) as libc::c_int
                                == 'e' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'x' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'x' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_EXPIREAT;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'b' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_BITCOUNT;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'g' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_GETRANGE;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SETRANGE;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SMEMBERS;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'v' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'v' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'k' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'k' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZREVRANK;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'b' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'f' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'f' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_BITFIELD;
                            }
                        }
                        9 => {
                            if (*m.offset(0 as libc::c_int as isize) as libc::c_int
                                == 'p' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'x' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'x' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_PEXPIREAT;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'r' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'p' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'p' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'h' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_RPOPLPUSH;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SISMEMBER;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'v' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'v' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZREVRANGE;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'x' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'x' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZLEXCOUNT;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'g' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'h' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_GEOSEARCH;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'g' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_GEORADIUS;
                            }
                        }
                        10 => {
                            if (*m.offset(0 as libc::c_int as isize) as libc::c_int
                                == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'f' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'f' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'f' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'f' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SDIFFSTORE;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'h' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'f' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'f' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_HRANDFIELD;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SMISMEMBER;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'f' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'f' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'f' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'f' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZDIFFSTORE;
                            }
                        }
                        11 => {
                            if (*m.offset(0 as libc::c_int as isize) as libc::c_int
                                == 'i' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'y' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'y' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'f' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'f' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(10 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(10 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_INCRBYFLOAT;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(10 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(10 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SINTERSTORE;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(10 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(10 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SRANDMEMBER;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 's' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(10 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(10 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_SUNIONSTORE;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(10 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(10 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZINTERSTORE;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(10 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(10 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZUNIONSTORE;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'y' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'y' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(10 as libc::c_int as isize) as libc::c_int
                                    == 'x' as i32
                                    || *m.offset(10 as libc::c_int as isize) as libc::c_int
                                        == 'x' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZRANGEBYLEX;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(10 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(10 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZRANDMEMBER;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(10 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(10 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZRANGESTORE;
                            }
                        }
                        12 => {
                            if (*m.offset(0 as libc::c_int as isize) as libc::c_int
                                == 'h' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'y' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'y' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'f' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'f' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(10 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(10 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(11 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(11 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_HINCRBYFLOAT;
                            }
                        }
                        13 => {
                            if (*m.offset(0 as libc::c_int as isize) as libc::c_int
                                == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'y' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'y' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(10 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(10 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(11 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(11 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(12 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(12 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZRANGEBYSCORE;
                            }
                        }
                        14 => {
                            if (*m.offset(0 as libc::c_int as isize) as libc::c_int
                                == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(10 as libc::c_int as isize) as libc::c_int
                                    == 'y' as i32
                                    || *m.offset(10 as libc::c_int as isize) as libc::c_int
                                        == 'y' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(11 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(11 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(12 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(12 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(13 as libc::c_int as isize) as libc::c_int
                                    == 'x' as i32
                                    || *m.offset(13 as libc::c_int as isize) as libc::c_int
                                        == 'x' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZREMRANGEBYLEX;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'v' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'v' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(10 as libc::c_int as isize) as libc::c_int
                                    == 'y' as i32
                                    || *m.offset(10 as libc::c_int as isize) as libc::c_int
                                        == 'y' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(11 as libc::c_int as isize) as libc::c_int
                                    == 'l' as i32
                                    || *m.offset(11 as libc::c_int as isize) as libc::c_int
                                        == 'l' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(12 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(12 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(13 as libc::c_int as isize) as libc::c_int
                                    == 'x' as i32
                                    || *m.offset(13 as libc::c_int as isize) as libc::c_int
                                        == 'x' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZREVRANGEBYLEX;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'g' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'h' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'h' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(10 as libc::c_int as isize) as libc::c_int
                                    == 't' as i32
                                    || *m.offset(10 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(11 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(11 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(12 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(12 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(13 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(13 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_GEOSEARCHSTORE;
                            }
                        }
                        15 => {
                            if (*m.offset(0 as libc::c_int as isize) as libc::c_int
                                == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(10 as libc::c_int as isize) as libc::c_int
                                    == 'y' as i32
                                    || *m.offset(10 as libc::c_int as isize) as libc::c_int
                                        == 'y' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(11 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(11 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(12 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(12 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(13 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(13 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(14 as libc::c_int as isize) as libc::c_int
                                    == 'k' as i32
                                    || *m.offset(14 as libc::c_int as isize) as libc::c_int
                                        == 'k' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZREMRANGEBYRANK;
                            }
                        }
                        16 => {
                            if (*m.offset(0 as libc::c_int as isize) as libc::c_int
                                == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(10 as libc::c_int as isize) as libc::c_int
                                    == 'y' as i32
                                    || *m.offset(10 as libc::c_int as isize) as libc::c_int
                                        == 'y' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(11 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(11 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(12 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(12 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(13 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(13 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(14 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(14 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(15 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(15 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZREMRANGEBYSCORE;
                            } else if (*m.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'z' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'z' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'v' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'v' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'n' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'n' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(10 as libc::c_int as isize) as libc::c_int
                                    == 'y' as i32
                                    || *m.offset(10 as libc::c_int as isize) as libc::c_int
                                        == 'y' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(11 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(11 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(12 as libc::c_int as isize) as libc::c_int
                                    == 'c' as i32
                                    || *m.offset(12 as libc::c_int as isize) as libc::c_int
                                        == 'c' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(13 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(13 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(14 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(14 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(15 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(15 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_ZREVRANGEBYSCORE;
                            }
                        }
                        17 => {
                            if (*m.offset(0 as libc::c_int as isize) as libc::c_int
                                == 'g' as i32
                                || *m.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 'g' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                    || *m.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 'o' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(3 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(3 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'a' as i32
                                    || *m.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 'a' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(5 as libc::c_int as isize) as libc::c_int
                                    == 'd' as i32
                                    || *m.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'd' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'i' as i32
                                    || *m.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'i' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(7 as libc::c_int as isize) as libc::c_int
                                    == 'u' as i32
                                    || *m.offset(7 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 's' as i32
                                    || *m.offset(8 as libc::c_int as isize) as libc::c_int
                                        == 's' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(9 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(9 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(10 as libc::c_int as isize) as libc::c_int
                                    == 'y' as i32
                                    || *m.offset(10 as libc::c_int as isize) as libc::c_int
                                        == 'y' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(11 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(11 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(12 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(12 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(13 as libc::c_int as isize) as libc::c_int
                                    == 'm' as i32
                                    || *m.offset(13 as libc::c_int as isize) as libc::c_int
                                        == 'm' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(14 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32
                                    || *m.offset(14 as libc::c_int as isize) as libc::c_int
                                        == 'b' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(15 as libc::c_int as isize) as libc::c_int
                                    == 'e' as i32
                                    || *m.offset(15 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32 ^ 0x20 as libc::c_int)
                                && (*m.offset(16 as libc::c_int as isize) as libc::c_int
                                    == 'r' as i32
                                    || *m.offset(16 as libc::c_int as isize) as libc::c_int
                                        == 'r' as i32 ^ 0x20 as libc::c_int)
                            {
                                (*r).type_0 = MSG_REQ_REDIS_GEORADIUSBYMEMBER;
                            }
                        }
                        _ => {}
                    }
                    if (*r).type_0 as libc::c_uint
                        == MSG_UNKNOWN as libc::c_int as libc::c_uint
                    {
                        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
                            _log(
                                b"nc_redis.c\0" as *const u8 as *const libc::c_char,
                                1324 as libc::c_int,
                                0 as libc::c_int,
                                b"parsed unsupported command '%.*s'\0" as *const u8
                                    as *const libc::c_char,
                                p.offset_from(m) as libc::c_long as libc::c_int,
                                m,
                            );
                        }
                        current_block = 9430895183509304024;
                        break;
                    } else {
                        state = SW_REQ_TYPE_LF;
                    }
                }
            }
            6 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 9430895183509304024;
                        break;
                    }
                }
                if redis_argz(r) {
                    if !((*r).narg != 1 as libc::c_int as libc::c_uint) {
                        current_block = 1526858968486531669;
                        break;
                    }
                    current_block = 9430895183509304024;
                    break;
                } else if redis_nokey(r) {
                    if (*r).narg == 1 as libc::c_int as libc::c_uint {
                        current_block = 1526858968486531669;
                        break;
                    }
                    state = SW_ARGN_LEN;
                } else {
                    if (*r).narg == 1 as libc::c_int as libc::c_uint {
                        current_block = 9430895183509304024;
                        break;
                    }
                    if redis_argeval(r) {
                        state = SW_ARG1_LEN;
                    } else {
                        state = SW_KEY_LEN;
                    }
                }
            }
            7 => {
                if ((*r).token).is_null() {
                    if ch as libc::c_int != '$' as i32 {
                        current_block = 9430895183509304024;
                        break;
                    }
                    (*r).token = p;
                    (*r).rlen = 0 as libc::c_int as uint32_t;
                } else if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    (*r)
                        .rlen = ((*r).rlen)
                        .wrapping_mul(10 as libc::c_int as libc::c_uint)
                        .wrapping_add((ch as libc::c_int - '0' as i32) as uint32_t);
                } else {
                    if !(ch as libc::c_int
                        == 13 as libc::c_int as uint8_t as libc::c_int)
                    {
                        current_block = 9430895183509304024;
                        break;
                    }
                    if (*r).rlen as libc::c_ulong >= mbuf_data_size() {
                        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
                            _log(
                                b"nc_redis.c\0" as *const u8 as *const libc::c_char,
                                1376 as libc::c_int,
                                0 as libc::c_int,
                                b"parsed bad req %lu of type %d with key length %d that greater than or equal to maximum redis key length of %zu\0"
                                    as *const u8 as *const libc::c_char,
                                (*r).id,
                                (*r).type_0 as libc::c_uint,
                                (*r).rlen,
                                mbuf_data_size(),
                            );
                        }
                        current_block = 9430895183509304024;
                        break;
                    } else {
                        if (*r).rnarg == 0 as libc::c_int as libc::c_uint {
                            current_block = 9430895183509304024;
                            break;
                        }
                        (*r).rnarg = ((*r).rnarg).wrapping_sub(1);
                        (*r).rnarg;
                        (*r).token = 0 as *mut uint8_t;
                        state = SW_KEY_LEN_LF;
                    }
                }
            }
            8 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 9430895183509304024;
                        break;
                    }
                }
                state = SW_KEY;
            }
            9 => {
                if ((*r).token).is_null() {
                    (*r).token = p;
                }
                m = ((*r).token).offset((*r).rlen as isize);
                if m >= (*b).last {
                    m = ((*b).last).offset(-(1 as libc::c_int as isize));
                    p = m;
                } else {
                    if *m as libc::c_int != 13 as libc::c_int as uint8_t as libc::c_int {
                        current_block = 9430895183509304024;
                        break;
                    }
                    let mut kpos: *mut keypos = 0 as *mut keypos;
                    p = m;
                    (*r).rlen = 0 as libc::c_int as uint32_t;
                    m = (*r).token;
                    (*r).token = 0 as *mut uint8_t;
                    kpos = array_push((*r).keys) as *mut keypos;
                    if kpos.is_null() {
                        current_block = 581218188993692030;
                        break;
                    }
                    (*kpos).start = m;
                    (*kpos).end = p;
                    state = SW_KEY_LF;
                }
            }
            10 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 9430895183509304024;
                        break;
                    }
                }
                if redis_arg0(r) {
                    if (*r).rnarg != 0 as libc::c_int as libc::c_uint {
                        current_block = 9430895183509304024;
                        break;
                    } else {
                        current_block = 1526858968486531669;
                        break;
                    }
                } else if redis_arg1(r) {
                    if (*r).rnarg != 1 as libc::c_int as libc::c_uint {
                        current_block = 9430895183509304024;
                        break;
                    }
                    state = SW_ARG1_LEN;
                } else if redis_arg2(r) {
                    if (*r).rnarg != 2 as libc::c_int as libc::c_uint {
                        current_block = 9430895183509304024;
                        break;
                    }
                    state = SW_ARG1_LEN;
                } else if redis_arg3(r) {
                    if (*r).rnarg != 3 as libc::c_int as libc::c_uint {
                        current_block = 9430895183509304024;
                        break;
                    }
                    state = SW_ARG1_LEN;
                } else if redis_argn(r) {
                    if (*r).rnarg == 0 as libc::c_int as libc::c_uint {
                        current_block = 1526858968486531669;
                        break;
                    }
                    state = SW_ARG1_LEN;
                } else if redis_argx(r) {
                    if (*r).rnarg == 0 as libc::c_int as libc::c_uint {
                        current_block = 1526858968486531669;
                        break;
                    }
                    state = SW_KEY_LEN;
                } else if redis_argkvx(r) {
                    if ((*r).narg).wrapping_rem(2 as libc::c_int as libc::c_uint)
                        == 0 as libc::c_int as libc::c_uint
                    {
                        current_block = 9430895183509304024;
                        break;
                    }
                    state = SW_ARG1_LEN;
                } else {
                    if !redis_argeval(r) {
                        current_block = 9430895183509304024;
                        break;
                    }
                    if (*r).rnarg == 0 as libc::c_int as libc::c_uint {
                        current_block = 1526858968486531669;
                        break;
                    }
                    state = SW_ARGN_LEN;
                }
            }
            11 => {
                if ((*r).token).is_null() {
                    if ch as libc::c_int != '$' as i32 {
                        current_block = 9430895183509304024;
                        break;
                    }
                    (*r).rlen = 0 as libc::c_int as uint32_t;
                    (*r).token = p;
                } else if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    (*r)
                        .rlen = ((*r).rlen)
                        .wrapping_mul(10 as libc::c_int as libc::c_uint)
                        .wrapping_add((ch as libc::c_int - '0' as i32) as uint32_t);
                } else {
                    if !(ch as libc::c_int
                        == 13 as libc::c_int as uint8_t as libc::c_int)
                    {
                        current_block = 9430895183509304024;
                        break;
                    }
                    if p.offset_from((*r).token) as libc::c_long
                        <= 1 as libc::c_int as libc::c_long
                        || (*r).rnarg == 0 as libc::c_int as libc::c_uint
                    {
                        current_block = 9430895183509304024;
                        break;
                    }
                    (*r).rnarg = ((*r).rnarg).wrapping_sub(1);
                    (*r).rnarg;
                    (*r).token = 0 as *mut uint8_t;
                    state = SW_ARG1_LEN_LF;
                }
            }
            12 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 9430895183509304024;
                        break;
                    }
                }
                state = SW_ARG1;
            }
            13 => {
                m = p.offset((*r).rlen as isize);
                if m >= (*b).last {
                    (*r)
                        .rlen = ((*r).rlen as libc::c_uint)
                        .wrapping_sub(
                            ((*b).last).offset_from(p) as libc::c_long as uint32_t,
                        ) as uint32_t as uint32_t;
                    m = ((*b).last).offset(-(1 as libc::c_int as isize));
                    p = m;
                } else {
                    if *m as libc::c_int != 13 as libc::c_int as uint8_t as libc::c_int {
                        current_block = 9430895183509304024;
                        break;
                    }
                    p = m;
                    (*r).rlen = 0 as libc::c_int as uint32_t;
                    state = SW_ARG1_LF;
                }
            }
            14 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 9430895183509304024;
                        break;
                    }
                }
                if redis_arg1(r) {
                    if (*r).rnarg != 0 as libc::c_int as libc::c_uint {
                        current_block = 9430895183509304024;
                        break;
                    } else {
                        current_block = 1526858968486531669;
                        break;
                    }
                } else if redis_arg2(r) {
                    if (*r).rnarg != 1 as libc::c_int as libc::c_uint {
                        current_block = 9430895183509304024;
                        break;
                    }
                    state = SW_ARG2_LEN;
                } else if redis_arg3(r) {
                    if (*r).rnarg != 2 as libc::c_int as libc::c_uint {
                        current_block = 9430895183509304024;
                        break;
                    }
                    state = SW_ARG2_LEN;
                } else if redis_argn(r) {
                    if (*r).rnarg == 0 as libc::c_int as libc::c_uint {
                        current_block = 1526858968486531669;
                        break;
                    }
                    state = SW_ARGN_LEN;
                } else if redis_argeval(r) {
                    if (*r).rnarg < 2 as libc::c_int as libc::c_uint {
                        current_block = 9430895183509304024;
                        break;
                    }
                    state = SW_ARG2_LEN;
                } else {
                    if !redis_argkvx(r) {
                        current_block = 9430895183509304024;
                        break;
                    }
                    if (*r).rnarg == 0 as libc::c_int as libc::c_uint {
                        current_block = 1526858968486531669;
                        break;
                    }
                    state = SW_KEY_LEN;
                }
            }
            15 => {
                if ((*r).token).is_null() {
                    if ch as libc::c_int != '$' as i32 {
                        current_block = 9430895183509304024;
                        break;
                    }
                    (*r).rlen = 0 as libc::c_int as uint32_t;
                    (*r).token = p;
                } else if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    (*r)
                        .rlen = ((*r).rlen)
                        .wrapping_mul(10 as libc::c_int as libc::c_uint)
                        .wrapping_add((ch as libc::c_int - '0' as i32) as uint32_t);
                } else {
                    if !(ch as libc::c_int
                        == 13 as libc::c_int as uint8_t as libc::c_int)
                    {
                        current_block = 9430895183509304024;
                        break;
                    }
                    if p.offset_from((*r).token) as libc::c_long
                        <= 1 as libc::c_int as libc::c_long
                        || (*r).rnarg == 0 as libc::c_int as libc::c_uint
                    {
                        current_block = 9430895183509304024;
                        break;
                    }
                    (*r).rnarg = ((*r).rnarg).wrapping_sub(1);
                    (*r).rnarg;
                    (*r).token = 0 as *mut uint8_t;
                    state = SW_ARG2_LEN_LF;
                }
            }
            16 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 9430895183509304024;
                        break;
                    }
                }
                state = SW_ARG2;
            }
            17 => {
                if ((*r).token).is_null() && redis_argeval(r) as libc::c_int != 0 {
                    (*r).token = p;
                }
                m = p.offset((*r).rlen as isize);
                if m >= (*b).last {
                    if ((*r).token).is_null() {
                        (*r)
                            .rlen = ((*r).rlen as libc::c_uint)
                            .wrapping_sub(
                                ((*b).last).offset_from(p) as libc::c_long as uint32_t,
                            ) as uint32_t as uint32_t;
                    }
                    m = ((*b).last).offset(-(1 as libc::c_int as isize));
                    p = m;
                } else {
                    if *m as libc::c_int != 13 as libc::c_int as uint8_t as libc::c_int {
                        current_block = 9430895183509304024;
                        break;
                    }
                    p = m;
                    (*r).rlen = 0 as libc::c_int as uint32_t;
                    if redis_argeval(r) {
                        let mut nkey: uint32_t = 0;
                        let mut chp: *mut uint8_t = 0 as *mut uint8_t;
                        if (p.offset_from((*r).token) as libc::c_long)
                            < 1 as libc::c_int as libc::c_long
                        {
                            current_block = 9430895183509304024;
                            break;
                        }
                        nkey = 0 as libc::c_int as uint32_t;
                        chp = (*r).token;
                        while chp < p {
                            if !(*(*__ctype_b_loc()).offset(*chp as libc::c_int as isize)
                                as libc::c_int
                                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                != 0)
                            {
                                current_block = 9430895183509304024;
                                break 's_34;
                            }
                            nkey = nkey
                                .wrapping_mul(10 as libc::c_int as libc::c_uint)
                                .wrapping_add(
                                    (*chp as libc::c_int - '0' as i32) as uint32_t,
                                );
                            chp = chp.offset(1);
                            chp;
                        }
                        if nkey == 0 as libc::c_int as libc::c_uint {
                            current_block = 9430895183509304024;
                            break;
                        }
                        (*r).token = 0 as *mut uint8_t;
                    }
                    state = SW_ARG2_LF;
                }
            }
            18 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 9430895183509304024;
                        break;
                    }
                }
                if redis_arg2(r) {
                    if (*r).rnarg != 0 as libc::c_int as libc::c_uint {
                        current_block = 9430895183509304024;
                        break;
                    } else {
                        current_block = 1526858968486531669;
                        break;
                    }
                } else if redis_arg3(r) {
                    if (*r).rnarg != 1 as libc::c_int as libc::c_uint {
                        current_block = 9430895183509304024;
                        break;
                    }
                    state = SW_ARG3_LEN;
                } else if redis_argn(r) {
                    if (*r).rnarg == 0 as libc::c_int as libc::c_uint {
                        current_block = 1526858968486531669;
                        break;
                    }
                    state = SW_ARGN_LEN;
                } else {
                    if !redis_argeval(r) {
                        current_block = 9430895183509304024;
                        break;
                    }
                    if (*r).rnarg < 1 as libc::c_int as libc::c_uint {
                        current_block = 9430895183509304024;
                        break;
                    }
                    state = SW_KEY_LEN;
                }
            }
            19 => {
                if ((*r).token).is_null() {
                    if ch as libc::c_int != '$' as i32 {
                        current_block = 9430895183509304024;
                        break;
                    }
                    (*r).rlen = 0 as libc::c_int as uint32_t;
                    (*r).token = p;
                } else if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    (*r)
                        .rlen = ((*r).rlen)
                        .wrapping_mul(10 as libc::c_int as libc::c_uint)
                        .wrapping_add((ch as libc::c_int - '0' as i32) as uint32_t);
                } else {
                    if !(ch as libc::c_int
                        == 13 as libc::c_int as uint8_t as libc::c_int)
                    {
                        current_block = 9430895183509304024;
                        break;
                    }
                    if p.offset_from((*r).token) as libc::c_long
                        <= 1 as libc::c_int as libc::c_long
                        || (*r).rnarg == 0 as libc::c_int as libc::c_uint
                    {
                        current_block = 9430895183509304024;
                        break;
                    }
                    (*r).rnarg = ((*r).rnarg).wrapping_sub(1);
                    (*r).rnarg;
                    (*r).token = 0 as *mut uint8_t;
                    state = SW_ARG3_LEN_LF;
                }
            }
            20 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 9430895183509304024;
                        break;
                    }
                }
                state = SW_ARG3;
            }
            21 => {
                m = p.offset((*r).rlen as isize);
                if m >= (*b).last {
                    (*r)
                        .rlen = ((*r).rlen as libc::c_uint)
                        .wrapping_sub(
                            ((*b).last).offset_from(p) as libc::c_long as uint32_t,
                        ) as uint32_t as uint32_t;
                    m = ((*b).last).offset(-(1 as libc::c_int as isize));
                    p = m;
                } else {
                    if *m as libc::c_int != 13 as libc::c_int as uint8_t as libc::c_int {
                        current_block = 9430895183509304024;
                        break;
                    }
                    p = m;
                    (*r).rlen = 0 as libc::c_int as uint32_t;
                    state = SW_ARG3_LF;
                }
            }
            22 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 9430895183509304024;
                        break;
                    }
                }
                if redis_arg3(r) {
                    if (*r).rnarg != 0 as libc::c_int as libc::c_uint {
                        current_block = 9430895183509304024;
                        break;
                    } else {
                        current_block = 1526858968486531669;
                        break;
                    }
                } else {
                    if !redis_argn(r) {
                        current_block = 9430895183509304024;
                        break;
                    }
                    if (*r).rnarg == 0 as libc::c_int as libc::c_uint {
                        current_block = 1526858968486531669;
                        break;
                    }
                    state = SW_ARGN_LEN;
                }
            }
            23 => {
                if ((*r).token).is_null() {
                    if ch as libc::c_int != '$' as i32 {
                        current_block = 9430895183509304024;
                        break;
                    }
                    (*r).rlen = 0 as libc::c_int as uint32_t;
                    (*r).token = p;
                } else if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    (*r)
                        .rlen = ((*r).rlen)
                        .wrapping_mul(10 as libc::c_int as libc::c_uint)
                        .wrapping_add((ch as libc::c_int - '0' as i32) as uint32_t);
                } else {
                    if !(ch as libc::c_int
                        == 13 as libc::c_int as uint8_t as libc::c_int)
                    {
                        current_block = 9430895183509304024;
                        break;
                    }
                    if p.offset_from((*r).token) as libc::c_long
                        <= 1 as libc::c_int as libc::c_long
                        || (*r).rnarg == 0 as libc::c_int as libc::c_uint
                    {
                        current_block = 9430895183509304024;
                        break;
                    }
                    (*r).rnarg = ((*r).rnarg).wrapping_sub(1);
                    (*r).rnarg;
                    (*r).token = 0 as *mut uint8_t;
                    state = SW_ARGN_LEN_LF;
                }
            }
            24 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 9430895183509304024;
                        break;
                    }
                }
                state = SW_ARGN;
            }
            25 => {
                m = p.offset((*r).rlen as isize);
                if m >= (*b).last {
                    (*r)
                        .rlen = ((*r).rlen as libc::c_uint)
                        .wrapping_sub(
                            ((*b).last).offset_from(p) as libc::c_long as uint32_t,
                        ) as uint32_t as uint32_t;
                    m = ((*b).last).offset(-(1 as libc::c_int as isize));
                    p = m;
                } else {
                    if *m as libc::c_int != 13 as libc::c_int as uint8_t as libc::c_int {
                        current_block = 9430895183509304024;
                        break;
                    }
                    p = m;
                    (*r).rlen = 0 as libc::c_int as uint32_t;
                    state = SW_ARGN_LF;
                }
            }
            26 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 9430895183509304024;
                        break;
                    }
                }
                if !(redis_argn(r) as libc::c_int != 0
                    || redis_argeval(r) as libc::c_int != 0
                    || redis_nokey(r) as libc::c_int != 0)
                {
                    current_block = 9430895183509304024;
                    break;
                }
                if (*r).rnarg == 0 as libc::c_int as libc::c_uint {
                    current_block = 1526858968486531669;
                    break;
                }
                state = SW_ARGN_LEN;
            }
            27 | _ => {}
        }
        p = p.offset(1);
        p;
    }
    match current_block {
        9430895183509304024 => {
            (*r).result = MSG_PARSE_ERROR;
            (*r).state = state as libc::c_int;
            *__errno_location() = 22 as libc::c_int;
            return;
        }
        1526858968486531669 => {
            (*r).pos = p.offset(1 as libc::c_int as isize);
            (*r).state = SW_START as libc::c_int;
            (*r).token = 0 as *mut uint8_t;
            (*r).result = MSG_PARSE_OK;
            return;
        }
        581218188993692030 => {
            (*r).result = MSG_PARSE_ERROR;
            (*r).state = state as libc::c_int;
            return;
        }
        _ => {
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
    };
}
pub unsafe extern "C" fn redis_parse_rsp(mut r: *mut msg) {
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
            current_block = 7147782476509834402;
            break;
        }
        ch = *p;
        match state as libc::c_uint {
            0 => {
                (*r).type_0 = MSG_UNKNOWN;
                (*r).rnarg = 1 as libc::c_int as uint32_t;
                (*r).is_top_level = 1 as libc::c_int as uint8_t;
                match ch as libc::c_int {
                    43 => {
                        p = p.offset(-(1 as libc::c_int as isize));
                        (*r).type_0 = MSG_RSP_REDIS_STATUS;
                        state = SW_STATUS;
                    }
                    45 => {
                        (*r).type_0 = MSG_RSP_REDIS_ERROR;
                        p = p.offset(-(1 as libc::c_int as isize));
                        state = SW_ERROR;
                    }
                    58 => {
                        (*r).type_0 = MSG_RSP_REDIS_INTEGER;
                        (*r).integer = 0 as libc::c_int as uint32_t;
                        state = SW_INTEGER_START;
                    }
                    36 => {
                        (*r).type_0 = MSG_RSP_REDIS_BULK;
                        p = p.offset(-(1 as libc::c_int as isize));
                        state = SW_BULK;
                    }
                    42 => {
                        (*r).type_0 = MSG_RSP_REDIS_MULTIBULK;
                        p = p.offset(-(1 as libc::c_int as isize));
                        state = SW_MULTIBULK;
                    }
                    _ => {
                        current_block = 11939663429262050696;
                        break;
                    }
                }
            }
            1 => {
                state = SW_RUNTO_CRLF;
            }
            2 => {
                if ((*r).token).is_null() {
                    if ch as libc::c_int != '-' as i32 {
                        current_block = 11939663429262050696;
                        break;
                    }
                    (*r).token = p;
                }
                if ch as libc::c_int == ' ' as i32
                    || ch as libc::c_int == 13 as libc::c_int as uint8_t as libc::c_int
                {
                    m = (*r).token;
                    (*r).token = 0 as *mut uint8_t;
                    match p.offset_from(m) as libc::c_long {
                        4 => {
                            if *(m as *mut uint32_t)
                                == (('R' as i32) << 24 as libc::c_int
                                    | ('R' as i32) << 16 as libc::c_int
                                    | ('E' as i32) << 8 as libc::c_int | '-' as i32)
                                    as libc::c_uint
                            {
                                (*r).type_0 = MSG_RSP_REDIS_ERROR_ERR;
                            } else if *(m as *mut uint32_t)
                                == (('M' as i32) << 24 as libc::c_int
                                    | ('O' as i32) << 16 as libc::c_int
                                    | ('O' as i32) << 8 as libc::c_int | '-' as i32)
                                    as libc::c_uint
                            {
                                (*r).type_0 = MSG_RSP_REDIS_ERROR_OOM;
                            }
                        }
                        5 => {
                            if *(m as *mut uint32_t)
                                == (('S' as i32) << 24 as libc::c_int
                                    | ('U' as i32) << 16 as libc::c_int
                                    | ('B' as i32) << 8 as libc::c_int | '-' as i32)
                                    as libc::c_uint
                                && *m.offset(4 as libc::c_int as isize) as libc::c_int
                                    == 'Y' as i32
                            {
                                (*r).type_0 = MSG_RSP_REDIS_ERROR_BUSY;
                            }
                        }
                        7 => {
                            if *(m as *mut uint32_t)
                                == (('A' as i32) << 24 as libc::c_int
                                    | ('O' as i32) << 16 as libc::c_int
                                    | ('N' as i32) << 8 as libc::c_int | '-' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    & 0xffff as libc::c_int as libc::c_uint
                                    == (('T' as i32) << 8 as libc::c_int | 'U' as i32)
                                        as libc::c_uint
                                && *m.offset(6 as libc::c_int as isize) as libc::c_int
                                    == 'H' as i32
                            {
                                (*r).type_0 = MSG_RSP_REDIS_ERROR_NOAUTH;
                            }
                        }
                        8 => {
                            if *(m as *mut uint32_t)
                                == (('A' as i32) << 24 as libc::c_int
                                    | ('O' as i32) << 16 as libc::c_int
                                    | ('L' as i32) << 8 as libc::c_int | '-' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    == (('G' as i32) << 24 as libc::c_int
                                        | ('N' as i32) << 16 as libc::c_int
                                        | ('I' as i32) << 8 as libc::c_int | 'D' as i32)
                                        as libc::c_uint
                            {
                                (*r).type_0 = MSG_RSP_REDIS_ERROR_LOADING;
                            } else if *(m as *mut uint32_t)
                                == (('S' as i32) << 24 as libc::c_int
                                    | ('U' as i32) << 16 as libc::c_int
                                    | ('B' as i32) << 8 as libc::c_int | '-' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    == (('Y' as i32) << 24 as libc::c_int
                                        | ('E' as i32) << 16 as libc::c_int
                                        | ('K' as i32) << 8 as libc::c_int | 'Y' as i32)
                                        as libc::c_uint
                            {
                                (*r).type_0 = MSG_RSP_REDIS_ERROR_BUSYKEY;
                            } else if *(m as *mut uint32_t)
                                == (('S' as i32) << 24 as libc::c_int
                                    | ('I' as i32) << 16 as libc::c_int
                                    | ('M' as i32) << 8 as libc::c_int | '-' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    == (('F' as i32) << 24 as libc::c_int
                                        | ('N' as i32) << 16 as libc::c_int
                                        | ('O' as i32) << 8 as libc::c_int | 'C' as i32)
                                        as libc::c_uint
                            {
                                (*r).type_0 = MSG_RSP_REDIS_ERROR_MISCONF;
                            }
                        }
                        9 => {
                            if *(m as *mut uint32_t)
                                == (('S' as i32) << 24 as libc::c_int
                                    | ('O' as i32) << 16 as libc::c_int
                                    | ('N' as i32) << 8 as libc::c_int | '-' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    == (('P' as i32) << 24 as libc::c_int
                                        | ('I' as i32) << 16 as libc::c_int
                                        | ('R' as i32) << 8 as libc::c_int | 'C' as i32)
                                        as libc::c_uint
                                && *m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'T' as i32
                            {
                                (*r).type_0 = MSG_RSP_REDIS_ERROR_NOSCRIPT;
                            } else if *(m as *mut uint32_t)
                                == (('A' as i32) << 24 as libc::c_int
                                    | ('E' as i32) << 16 as libc::c_int
                                    | ('R' as i32) << 8 as libc::c_int | '-' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    == (('L' as i32) << 24 as libc::c_int
                                        | ('N' as i32) << 16 as libc::c_int
                                        | ('O' as i32) << 8 as libc::c_int | 'D' as i32)
                                        as libc::c_uint
                                && *m.offset(8 as libc::c_int as isize) as libc::c_int
                                    == 'Y' as i32
                            {
                                (*r).type_0 = MSG_RSP_REDIS_ERROR_READONLY;
                            }
                        }
                        10 => {
                            if *(m as *mut uint32_t)
                                == (('O' as i32) << 24 as libc::c_int
                                    | ('R' as i32) << 16 as libc::c_int
                                    | ('W' as i32) << 8 as libc::c_int | '-' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    == (('Y' as i32) << 24 as libc::c_int
                                        | ('T' as i32) << 16 as libc::c_int
                                        | ('G' as i32) << 8 as libc::c_int | 'N' as i32)
                                        as libc::c_uint
                                && *(m as *mut uint32_t).offset(2 as libc::c_int as isize)
                                    & 0xffff as libc::c_int as libc::c_uint
                                    == (('E' as i32) << 8 as libc::c_int | 'P' as i32)
                                        as libc::c_uint
                            {
                                (*r).type_0 = MSG_RSP_REDIS_ERROR_WRONGTYPE;
                            } else if *(m as *mut uint32_t)
                                == (('E' as i32) << 24 as libc::c_int
                                    | ('X' as i32) << 16 as libc::c_int
                                    | ('E' as i32) << 8 as libc::c_int | '-' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    == (('O' as i32) << 24 as libc::c_int
                                        | ('B' as i32) << 16 as libc::c_int
                                        | ('A' as i32) << 8 as libc::c_int | 'C' as i32)
                                        as libc::c_uint
                                && *(m as *mut uint32_t).offset(2 as libc::c_int as isize)
                                    & 0xffff as libc::c_int as libc::c_uint
                                    == (('T' as i32) << 8 as libc::c_int | 'R' as i32)
                                        as libc::c_uint
                            {
                                (*r).type_0 = MSG_RSP_REDIS_ERROR_EXECABORT;
                            }
                        }
                        11 => {
                            if *(m as *mut uint32_t)
                                == (('S' as i32) << 24 as libc::c_int
                                    | ('A' as i32) << 16 as libc::c_int
                                    | ('M' as i32) << 8 as libc::c_int | '-' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    == (('D' as i32) << 24 as libc::c_int
                                        | ('R' as i32) << 16 as libc::c_int
                                        | ('E' as i32) << 8 as libc::c_int | 'T' as i32)
                                        as libc::c_uint
                                && *(m as *mut uint32_t).offset(2 as libc::c_int as isize)
                                    & 0xffff as libc::c_int as libc::c_uint
                                    == (('W' as i32) << 8 as libc::c_int | 'O' as i32)
                                        as libc::c_uint
                                && *m.offset(10 as libc::c_int as isize) as libc::c_int
                                    == 'N' as i32
                            {
                                (*r).type_0 = MSG_RSP_REDIS_ERROR_MASTERDOWN;
                            } else if *(m as *mut uint32_t)
                                == (('R' as i32) << 24 as libc::c_int
                                    | ('O' as i32) << 16 as libc::c_int
                                    | ('N' as i32) << 8 as libc::c_int | '-' as i32)
                                    as libc::c_uint
                                && *(m as *mut uint32_t).offset(1 as libc::c_int as isize)
                                    == (('I' as i32) << 24 as libc::c_int
                                        | ('L' as i32) << 16 as libc::c_int
                                        | ('P' as i32) << 8 as libc::c_int | 'E' as i32)
                                        as libc::c_uint
                                && *(m as *mut uint32_t).offset(2 as libc::c_int as isize)
                                    & 0xffff as libc::c_int as libc::c_uint
                                    == (('A' as i32) << 8 as libc::c_int | 'C' as i32)
                                        as libc::c_uint
                                && *m.offset(10 as libc::c_int as isize) as libc::c_int
                                    == 'S' as i32
                            {
                                (*r).type_0 = MSG_RSP_REDIS_ERROR_NOREPLICAS;
                            }
                        }
                        _ => {}
                    }
                    if ch as libc::c_int == '\r' as i32 {
                        state = SW_ALMOST_DONE;
                    } else {
                        state = SW_RUNTO_CRLF;
                    }
                }
            }
            4 => {
                if ch as libc::c_int == 13 as libc::c_int as uint8_t as libc::c_int {
                    (*r).rnarg = ((*r).rnarg).wrapping_sub(1);
                    (*r).rnarg;
                    state = SW_MULTIBULK_ARGN_LF;
                }
            }
            3 => {
                if ch as libc::c_int == 13 as libc::c_int as uint8_t as libc::c_int {
                    state = SW_ALMOST_DONE;
                } else if !(ch as libc::c_int == '-' as i32) {
                    if !(*(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
                    {
                        current_block = 11939663429262050696;
                        break;
                    }
                    (*r)
                        .integer = ((*r).integer)
                        .wrapping_mul(10 as libc::c_int as libc::c_uint)
                        .wrapping_add((ch as libc::c_int - '0' as i32) as uint32_t);
                }
            }
            15 => {
                match ch as libc::c_int {
                    13 => {
                        state = SW_ALMOST_DONE;
                    }
                    _ => {}
                }
            }
            16 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 11939663429262050696;
                        break;
                    }
                }
                current_block = 16477797002856340645;
                break;
            }
            5 => {
                if ((*r).token).is_null() {
                    if ch as libc::c_int != '$' as i32 {
                        current_block = 11939663429262050696;
                        break;
                    }
                    (*r).token = p;
                    (*r).rlen = 0 as libc::c_int as uint32_t;
                } else if ch as libc::c_int == '-' as i32 {
                    state = SW_RUNTO_CRLF;
                } else if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    (*r)
                        .rlen = ((*r).rlen)
                        .wrapping_mul(10 as libc::c_int as libc::c_uint)
                        .wrapping_add((ch as libc::c_int - '0' as i32) as uint32_t);
                } else {
                    if !(ch as libc::c_int
                        == 13 as libc::c_int as uint8_t as libc::c_int)
                    {
                        current_block = 11939663429262050696;
                        break;
                    }
                    if p.offset_from((*r).token) as libc::c_long
                        <= 1 as libc::c_int as libc::c_long
                    {
                        current_block = 11939663429262050696;
                        break;
                    }
                    (*r).token = 0 as *mut uint8_t;
                    state = SW_BULK_LF;
                }
            }
            6 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 11939663429262050696;
                        break;
                    }
                }
                state = SW_BULK_ARG;
            }
            7 => {
                m = p.offset((*r).rlen as isize);
                if m >= (*b).last {
                    (*r)
                        .rlen = ((*r).rlen as libc::c_uint)
                        .wrapping_sub(
                            ((*b).last).offset_from(p) as libc::c_long as uint32_t,
                        ) as uint32_t as uint32_t;
                    m = ((*b).last).offset(-(1 as libc::c_int as isize));
                    p = m;
                } else {
                    if *m as libc::c_int != 13 as libc::c_int as uint8_t as libc::c_int {
                        current_block = 11939663429262050696;
                        break;
                    }
                    p = m;
                    (*r).rlen = 0 as libc::c_int as uint32_t;
                    state = SW_BULK_ARG_LF;
                }
            }
            8 => {
                match ch as libc::c_int {
                    10 => {
                        current_block = 16477797002856340645;
                        break;
                    }
                    _ => {
                        current_block = 11939663429262050696;
                        break;
                    }
                }
            }
            9 => {
                if ((*r).token).is_null() {
                    if ch as libc::c_int != '*' as i32 {
                        current_block = 11939663429262050696;
                        break;
                    }
                    (*r).vlen = 0 as libc::c_int as uint32_t;
                    (*r).token = p;
                    if (*r).is_top_level != 0 {
                        (*r).narg_start = p;
                    }
                } else if ch as libc::c_int == '-' as i32 {
                    p = p.offset(-(1 as libc::c_int as isize));
                    (*r).token = 0 as *mut uint8_t;
                    (*r).vlen = 1 as libc::c_int as uint32_t;
                    state = SW_MULTIBULK_ARGN_LEN;
                } else if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    (*r)
                        .vlen = ((*r).vlen)
                        .wrapping_mul(10 as libc::c_int as libc::c_uint)
                        .wrapping_add((ch as libc::c_int - '0' as i32) as uint32_t);
                } else {
                    if !(ch as libc::c_int
                        == 13 as libc::c_int as uint8_t as libc::c_int)
                    {
                        current_block = 11939663429262050696;
                        break;
                    }
                    if p.offset_from((*r).token) as libc::c_long
                        <= 1 as libc::c_int as libc::c_long
                    {
                        current_block = 11939663429262050696;
                        break;
                    }
                    if (*r).is_top_level != 0 {
                        (*r).narg = (*r).vlen;
                        (*r).narg_end = p;
                    }
                    (*r).is_top_level = 0 as libc::c_int as uint8_t;
                    (*r)
                        .rnarg = ((*r).rnarg as libc::c_uint)
                        .wrapping_add(
                            ((*r).vlen).wrapping_sub(1 as libc::c_int as libc::c_uint),
                        ) as uint32_t as uint32_t;
                    (*r).token = 0 as *mut uint8_t;
                    state = SW_MULTIBULK_NARG_LF;
                }
            }
            10 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 11939663429262050696;
                        break;
                    }
                }
                if (*r).rnarg == 0 as libc::c_int as libc::c_uint {
                    current_block = 16477797002856340645;
                    break;
                }
                state = SW_MULTIBULK_ARGN_LEN;
            }
            11 => {
                if ((*r).token).is_null() {
                    if ch as libc::c_int == '*' as i32 {
                        p = p.offset(-(1 as libc::c_int as isize));
                        state = SW_MULTIBULK;
                    } else if ch as libc::c_int == ':' as i32
                        || ch as libc::c_int == '+' as i32
                        || ch as libc::c_int == '-' as i32
                    {
                        state = SW_SIMPLE;
                    } else {
                        if ch as libc::c_int != '$' as i32 {
                            current_block = 11939663429262050696;
                            break;
                        }
                        (*r).token = p;
                        (*r).rlen = 0 as libc::c_int as uint32_t;
                    }
                } else if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    (*r)
                        .rlen = ((*r).rlen)
                        .wrapping_mul(10 as libc::c_int as libc::c_uint)
                        .wrapping_add((ch as libc::c_int - '0' as i32) as uint32_t);
                } else if !(ch as libc::c_int == '-' as i32) {
                    if !(ch as libc::c_int
                        == 13 as libc::c_int as uint8_t as libc::c_int)
                    {
                        current_block = 11939663429262050696;
                        break;
                    }
                    if p.offset_from((*r).token) as libc::c_long
                        <= 1 as libc::c_int as libc::c_long
                        || (*r).rnarg == 0 as libc::c_int as libc::c_uint
                    {
                        current_block = 11939663429262050696;
                        break;
                    }
                    if (*r).rlen == 1 as libc::c_int as libc::c_uint
                        && p.offset_from((*r).token) as libc::c_long
                            == 3 as libc::c_int as libc::c_long
                    {
                        (*r).rlen = 0 as libc::c_int as uint32_t;
                        state = SW_MULTIBULK_ARGN_LF;
                    } else {
                        state = SW_MULTIBULK_ARGN_LEN_LF;
                    }
                    (*r).rnarg = ((*r).rnarg).wrapping_sub(1);
                    (*r).rnarg;
                    (*r).token = 0 as *mut uint8_t;
                }
            }
            12 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 11939663429262050696;
                        break;
                    }
                }
                state = SW_MULTIBULK_ARGN;
            }
            13 => {
                m = p.offset((*r).rlen as isize);
                if m >= (*b).last {
                    (*r)
                        .rlen = ((*r).rlen as libc::c_uint)
                        .wrapping_sub(
                            ((*b).last).offset_from(p) as libc::c_long as uint32_t,
                        ) as uint32_t as uint32_t;
                    m = ((*b).last).offset(-(1 as libc::c_int as isize));
                    p = m;
                } else {
                    if *m as libc::c_int != 13 as libc::c_int as uint8_t as libc::c_int {
                        current_block = 11939663429262050696;
                        break;
                    }
                    p = p.offset((*r).rlen as isize);
                    (*r).rlen = 0 as libc::c_int as uint32_t;
                    state = SW_MULTIBULK_ARGN_LF;
                }
            }
            14 => {
                match ch as libc::c_int {
                    10 => {}
                    _ => {
                        current_block = 11939663429262050696;
                        break;
                    }
                }
                if (*r).rnarg == 0 as libc::c_int as libc::c_uint {
                    current_block = 16477797002856340645;
                    break;
                }
                state = SW_MULTIBULK_ARGN_LEN;
            }
            17 | _ => {}
        }
        p = p.offset(1);
        p;
    }
    match current_block {
        11939663429262050696 => {
            (*r).result = MSG_PARSE_ERROR;
            (*r).state = state as libc::c_int;
            *__errno_location() = 22 as libc::c_int;
            return;
        }
        16477797002856340645 => {
            (*r).pos = p.offset(1 as libc::c_int as isize);
            (*r).state = SW_START_0 as libc::c_int;
            (*r).token = 0 as *mut uint8_t;
            (*r).result = MSG_PARSE_OK;
            return;
        }
        _ => {
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
    };
}
pub unsafe extern "C" fn redis_failure(mut r: *const msg) -> bool {
    match (*r).type_0 as libc::c_uint {
        169 | 170 | 172 => return 1 as libc::c_int != 0,
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn redis_copy_bulk(mut dst: *mut msg, mut src: *mut msg) -> rstatus_t {
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    let mut nbuf: *mut mbuf = 0 as *mut mbuf;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut len: uint32_t = 0 as libc::c_int as uint32_t;
    let mut bytes: uint32_t = 0 as libc::c_int as uint32_t;
    let mut status: rstatus_t = 0;
    mbuf = (*src).mhdr.stqh_first;
    while !mbuf.is_null() && mbuf_empty(mbuf) as libc::c_int != 0 {
        mbuf_remove(&mut (*src).mhdr, mbuf);
        mbuf_put(mbuf);
        mbuf = (*src).mhdr.stqh_first;
    }
    mbuf = (*src).mhdr.stqh_first;
    if mbuf.is_null() {
        return -(1 as libc::c_int);
    }
    p = (*mbuf).pos;
    p = p.offset(1);
    p;
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
        && *p.offset(1 as libc::c_int as isize) as libc::c_int == '1' as i32
    {
        len = ((1 as libc::c_int + 2 as libc::c_int) as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) as uint32_t;
        p = ((*mbuf).pos).offset(len as isize);
    } else {
        len = 0 as libc::c_int as uint32_t;
        while p < (*mbuf).last
            && *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            len = len
                .wrapping_mul(10 as libc::c_int as libc::c_uint)
                .wrapping_add((*p as libc::c_int - '0' as i32) as uint32_t);
            p = p.offset(1);
            p;
        }
        len = (len as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong),
            ) as uint32_t as uint32_t;
        len = (len as libc::c_long + p.offset_from((*mbuf).pos) as libc::c_long)
            as uint32_t;
    }
    bytes = len;
    while !mbuf.is_null() {
        if mbuf_length(mbuf) <= len {
            nbuf = (*mbuf).next.stqe_next;
            mbuf_remove(&mut (*src).mhdr, mbuf);
            if !dst.is_null() {
                mbuf_insert(&mut (*dst).mhdr, mbuf);
            } else {
                mbuf_put(mbuf);
            }
            len = (len as libc::c_uint).wrapping_sub(mbuf_length(mbuf)) as uint32_t
                as uint32_t;
            mbuf = nbuf;
        } else {
            if !dst.is_null() {
                status = msg_append(dst, (*mbuf).pos, len as size_t);
                if status != 0 as libc::c_int {
                    return status;
                }
            }
            (*mbuf).pos = ((*mbuf).pos).offset(len as isize);
            break;
        }
    }
    if !dst.is_null() {
        (*dst)
            .mlen = ((*dst).mlen as libc::c_uint).wrapping_add(bytes) as uint32_t
            as uint32_t;
    }
    (*src)
        .mlen = ((*src).mlen as libc::c_uint).wrapping_sub(bytes) as uint32_t
        as uint32_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn redis_pre_coalesce(mut r: *mut msg) {
    let mut pr: *mut msg = (*r).peer;
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    if (*pr).frag_id == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    (*(*pr).frag_owner).nfrag_done = ((*(*pr).frag_owner).nfrag_done).wrapping_add(1);
    (*(*pr).frag_owner).nfrag_done;
    match (*r).type_0 as libc::c_uint {
        181 => {
            mbuf = (*r).mhdr.stqh_first;
            (*r)
                .mlen = ((*r).mlen as libc::c_uint).wrapping_sub(mbuf_length(mbuf))
                as uint32_t as uint32_t;
            mbuf_rewind(mbuf);
            (*(*pr).frag_owner)
                .integer = ((*(*pr).frag_owner).integer as libc::c_uint)
                .wrapping_add((*r).integer) as uint32_t as uint32_t;
        }
        183 => {
            mbuf = (*r).mhdr.stqh_first;
            (*r)
                .narg_end = ((*r).narg_end)
                .offset(
                    (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                );
            (*r)
                .mlen = ((*r).mlen as libc::c_uint)
                .wrapping_sub(
                    ((*r).narg_end).offset_from((*r).narg_start) as libc::c_long
                        as uint32_t,
                ) as uint32_t as uint32_t;
            (*mbuf).pos = (*r).narg_end;
        }
        166 => {
            if (*pr).type_0 as libc::c_uint
                == MSG_REQ_REDIS_MSET as libc::c_int as libc::c_uint
            {
                mbuf = (*r).mhdr.stqh_first;
                (*r)
                    .mlen = ((*r).mlen as libc::c_uint).wrapping_sub(mbuf_length(mbuf))
                    as uint32_t as uint32_t;
                mbuf_rewind(mbuf);
            }
        }
        _ => {
            mbuf = (*r).mhdr.stqh_first;
            (*pr).set_error(1 as libc::c_int as libc::c_uint);
            (*pr).err = 22 as libc::c_int;
        }
    };
}
unsafe extern "C" fn redis_append_key(
    mut r: *mut msg,
    mut key: *const uint8_t,
    mut keylen: uint32_t,
) -> rstatus_t {
    let mut len: uint32_t = 0;
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    let mut printbuf: [uint8_t; 32] = [0; 32];
    let mut kpos: *mut keypos = 0 as *mut keypos;
    len = snprintf(
        printbuf.as_mut_ptr() as *mut libc::c_char,
        ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong,
        b"$%d\r\n\0" as *const u8 as *const libc::c_char,
        keylen,
    ) as uint32_t;
    mbuf = msg_ensure_mbuf(r, len as size_t);
    if mbuf.is_null() {
        return -(3 as libc::c_int);
    }
    mbuf_copy(mbuf, printbuf.as_mut_ptr(), len as size_t);
    (*r).mlen = ((*r).mlen as libc::c_uint).wrapping_add(len) as uint32_t as uint32_t;
    mbuf = msg_ensure_mbuf(r, keylen as size_t);
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
    mbuf = msg_ensure_mbuf(
        r,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if mbuf.is_null() {
        return -(3 as libc::c_int);
    }
    mbuf_copy(
        mbuf,
        b"\r\n\0" as *const u8 as *const libc::c_char as *mut uint8_t,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    (*r)
        .mlen = ((*r).mlen as libc::c_uint)
        .wrapping_add(
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
        ) as uint32_t as uint32_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn redis_fragment_argx(
    mut r: *mut msg,
    mut nserver: uint32_t,
    mut frag_msgq: *mut msg_tqh,
    mut key_step: uint32_t,
) -> rstatus_t {
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    let mut sub_msgs: *mut *mut msg = 0 as *mut *mut msg;
    let mut i: uint32_t = 0;
    let mut status: rstatus_t = 0;
    let mut keys: *mut array = (*r).keys;
    sub_msgs = _nc_zalloc(
        (nserver as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut msg>() as libc::c_ulong),
        b"nc_redis.c\0" as *const u8 as *const libc::c_char,
        2829 as libc::c_int,
    ) as *mut *mut msg;
    if sub_msgs.is_null() {
        return -(3 as libc::c_int);
    }
    (*r)
        .frag_seq = _nc_alloc(
        (array_n(keys) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut msg>() as libc::c_ulong),
        b"nc_redis.c\0" as *const u8 as *const libc::c_char,
        2835 as libc::c_int,
    ) as *mut *mut msg;
    if ((*r).frag_seq).is_null() {
        _nc_free(
            sub_msgs as *mut libc::c_void,
            b"nc_redis.c\0" as *const u8 as *const libc::c_char,
            2837 as libc::c_int,
        );
        sub_msgs = 0 as *mut *mut msg;
        return -(3 as libc::c_int);
    }
    mbuf = (*r).mhdr.stqh_first;
    (*mbuf).pos = (*mbuf).start;
    i = 0 as libc::c_int as uint32_t;
    while i < 3 as libc::c_int as libc::c_uint {
        while *(*mbuf).pos as libc::c_int != '\n' as i32 {
            (*mbuf).pos = ((*mbuf).pos).offset(1);
            (*mbuf).pos;
        }
        (*mbuf).pos = ((*mbuf).pos).offset(1);
        (*mbuf).pos;
        i = i.wrapping_add(1);
        i;
    }
    (*r).frag_id = msg_gen_frag_id();
    (*r).nfrag = 0 as libc::c_int as uint32_t;
    (*r).frag_owner = r;
    i = 0 as libc::c_int as uint32_t;
    while i < array_n(keys) {
        let mut sub_msg: *mut msg = 0 as *mut msg;
        let mut kpos: *mut keypos = array_get(keys, i) as *mut keypos;
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
                    b"nc_redis.c\0" as *const u8 as *const libc::c_char,
                    2871 as libc::c_int,
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
        status = redis_append_key(
            sub_msg,
            (*kpos).start,
            ((*kpos).end).offset_from((*kpos).start) as libc::c_long as uint32_t,
        );
        if status != 0 as libc::c_int {
            _nc_free(
                sub_msgs as *mut libc::c_void,
                b"nc_redis.c\0" as *const u8 as *const libc::c_char,
                2880 as libc::c_int,
            );
            sub_msgs = 0 as *mut *mut msg;
            return status;
        }
        if !(key_step == 1 as libc::c_int as libc::c_uint) {
            status = redis_copy_bulk(0 as *mut msg, r);
            if status != 0 as libc::c_int {
                _nc_free(
                    sub_msgs as *mut libc::c_void,
                    b"nc_redis.c\0" as *const u8 as *const libc::c_char,
                    2889 as libc::c_int,
                );
                sub_msgs = 0 as *mut *mut msg;
                return status;
            }
            status = redis_copy_bulk(sub_msg, r);
            if status != 0 as libc::c_int {
                _nc_free(
                    sub_msgs as *mut libc::c_void,
                    b"nc_redis.c\0" as *const u8 as *const libc::c_char,
                    2895 as libc::c_int,
                );
                sub_msgs = 0 as *mut *mut msg;
                return status;
            }
            (*sub_msg).narg = ((*sub_msg).narg).wrapping_add(1);
            (*sub_msg).narg;
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as uint32_t;
    while i < nserver {
        let mut sub_msg_0: *mut msg = *sub_msgs.offset(i as isize);
        if !sub_msg_0.is_null() {
            if (*r).type_0 as libc::c_uint
                == MSG_REQ_REDIS_MGET as libc::c_int as libc::c_uint
            {
                status = msg_prepend_format(
                    sub_msg_0,
                    b"*%d\r\n$4\r\nmget\r\n\0" as *const u8 as *const libc::c_char,
                    ((*sub_msg_0).narg).wrapping_add(1 as libc::c_int as libc::c_uint),
                );
            } else if (*r).type_0 as libc::c_uint
                == MSG_REQ_REDIS_DEL as libc::c_int as libc::c_uint
            {
                status = msg_prepend_format(
                    sub_msg_0,
                    b"*%d\r\n$3\r\ndel\r\n\0" as *const u8 as *const libc::c_char,
                    ((*sub_msg_0).narg).wrapping_add(1 as libc::c_int as libc::c_uint),
                );
            } else if (*r).type_0 as libc::c_uint
                == MSG_REQ_REDIS_MSET as libc::c_int as libc::c_uint
            {
                status = msg_prepend_format(
                    sub_msg_0,
                    b"*%d\r\n$4\r\nmset\r\n\0" as *const u8 as *const libc::c_char,
                    ((*sub_msg_0).narg).wrapping_add(1 as libc::c_int as libc::c_uint),
                );
            } else if (*r).type_0 as libc::c_uint
                == MSG_REQ_REDIS_TOUCH as libc::c_int as libc::c_uint
            {
                status = msg_prepend_format(
                    sub_msg_0,
                    b"*%d\r\n$5\r\ntouch\r\n\0" as *const u8 as *const libc::c_char,
                    ((*sub_msg_0).narg).wrapping_add(1 as libc::c_int as libc::c_uint),
                );
            } else if (*r).type_0 as libc::c_uint
                == MSG_REQ_REDIS_UNLINK as libc::c_int as libc::c_uint
            {
                status = msg_prepend_format(
                    sub_msg_0,
                    b"*%d\r\n$6\r\nunlink\r\n\0" as *const u8 as *const libc::c_char,
                    ((*sub_msg_0).narg).wrapping_add(1 as libc::c_int as libc::c_uint),
                );
            }
            if status != 0 as libc::c_int {
                _nc_free(
                    sub_msgs as *mut libc::c_void,
                    b"nc_redis.c\0" as *const u8 as *const libc::c_char,
                    2932 as libc::c_int,
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
        b"nc_redis.c\0" as *const u8 as *const libc::c_char,
        2944 as libc::c_int,
    );
    sub_msgs = 0 as *mut *mut msg;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn redis_fragment(
    mut r: *mut msg,
    mut nserver: uint32_t,
    mut frag_msgq: *mut msg_tqh,
) -> rstatus_t {
    if 1 as libc::c_int as libc::c_uint == array_n((*r).keys) {
        return 0 as libc::c_int;
    }
    match (*r).type_0 as libc::c_uint {
        59 | 29 | 39 | 42 => {
            return redis_fragment_argx(
                r,
                nserver,
                frag_msgq,
                1 as libc::c_int as uint32_t,
            );
        }
        60 => {
            return redis_fragment_argx(
                r,
                nserver,
                frag_msgq,
                2 as libc::c_int as uint32_t,
            );
        }
        _ => return 0 as libc::c_int,
    };
}
pub unsafe extern "C" fn redis_reply(mut r: *mut msg) -> rstatus_t {
    let mut c_conn: *mut conn = 0 as *mut conn;
    let mut response: *mut msg = (*r).peer;
    c_conn = (*response).owner;
    if (*r).type_0 as libc::c_uint == MSG_REQ_REDIS_AUTH as libc::c_int as libc::c_uint {
        return redis_handle_auth_req(r, response);
    }
    if !conn_authenticated(c_conn) {
        return msg_append(
            response,
            rsp_auth_required.data,
            rsp_auth_required.len as size_t,
        );
    }
    match (*r).type_0 as libc::c_uint {
        160 => return msg_append(response, rsp_pong.data, rsp_pong.len as size_t),
        _ => return -(1 as libc::c_int),
    };
}
pub unsafe extern "C" fn redis_post_coalesce_mset(mut request: *mut msg) {
    let mut status: rstatus_t = 0;
    let mut response: *mut msg = (*request).peer;
    status = msg_append(response, rsp_ok.data, rsp_ok.len as size_t);
    if status != 0 as libc::c_int {
        (*response).set_error(1 as libc::c_int as libc::c_uint);
        (*response).err = *__errno_location();
    }
}
pub unsafe extern "C" fn redis_post_coalesce_del_or_touch(mut request: *mut msg) {
    let mut response: *mut msg = (*request).peer;
    let mut status: rstatus_t = 0;
    status = msg_prepend_format(
        response,
        b":%d\r\n\0" as *const u8 as *const libc::c_char,
        (*request).integer,
    );
    if status != 0 as libc::c_int {
        (*response).set_error(1 as libc::c_int as libc::c_uint);
        (*response).err = *__errno_location();
    }
}
unsafe extern "C" fn redis_post_coalesce_mget(mut request: *mut msg) {
    let mut response: *mut msg = (*request).peer;
    let mut sub_msg: *mut msg = 0 as *mut msg;
    let mut status: rstatus_t = 0;
    let mut i: uint32_t = 0;
    status = msg_prepend_format(
        response,
        b"*%d\r\n\0" as *const u8 as *const libc::c_char,
        ((*request).narg).wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if status != 0 as libc::c_int {
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
        status = redis_copy_bulk(response, sub_msg);
        if status != 0 as libc::c_int {
            (*(*response).owner).err = 1 as libc::c_int;
            return;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn redis_post_coalesce(mut r: *mut msg) {
    let mut pr: *mut msg = (*r).peer;
    if (*r).error() as libc::c_int != 0 || (*r).ferror() as libc::c_int != 0 {
        return;
    }
    match (*r).type_0 as libc::c_uint {
        59 => return redis_post_coalesce_mget(r),
        29 | 39 | 42 => return redis_post_coalesce_del_or_touch(r),
        60 => return redis_post_coalesce_mset(r),
        _ => {}
    };
}
unsafe extern "C" fn redis_handle_auth_req(
    mut req: *mut msg,
    mut rsp: *mut msg,
) -> rstatus_t {
    let mut conn: *mut conn = (*rsp).owner;
    let mut pool: *const server_pool = 0 as *const server_pool;
    let mut kpos: *const keypos = 0 as *const keypos;
    let mut key: *const uint8_t = 0 as *const uint8_t;
    let mut keylen: uint32_t = 0;
    let mut valid: bool = false;
    pool = (*conn).owner as *const server_pool;
    if (*pool).require_auth == 0 {
        return msg_append(rsp, rsp_no_password.data, rsp_no_password.len as size_t);
    }
    kpos = array_get((*req).keys, 0 as libc::c_int as uint32_t) as *const keypos;
    key = (*kpos).start;
    keylen = ((*kpos).end).offset_from((*kpos).start) as libc::c_long as uint32_t;
    valid = keylen == (*pool).redis_auth.len
        && memcmp(
            (*pool).redis_auth.data as *const libc::c_void,
            key as *const libc::c_void,
            keylen as libc::c_ulong,
        ) == 0 as libc::c_int;
    if valid {
        (*conn).set_authenticated(1 as libc::c_int as libc::c_uint);
        return msg_append(rsp, rsp_ok.data, rsp_ok.len as size_t);
    }
    (*conn).set_authenticated(0 as libc::c_int as libc::c_uint);
    return msg_append(
        rsp,
        rsp_invalid_password.data,
        rsp_invalid_password.len as size_t,
    );
}
pub unsafe extern "C" fn redis_add_auth(
    mut ctx: *mut context,
    mut c_conn: *mut conn,
    mut s_conn: *mut conn,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut msg: *mut msg = 0 as *mut msg;
    let mut pool: *mut server_pool = 0 as *mut server_pool;
    pool = (*c_conn).owner as *mut server_pool;
    msg = msg_get(c_conn, 1 as libc::c_int != 0, (*c_conn).redis() != 0);
    if msg.is_null() {
        (*c_conn).err = *__errno_location();
        return -(3 as libc::c_int);
    }
    status = msg_prepend_format(
        msg,
        b"*2\r\n$4\r\nAUTH\r\n$%d\r\n%s\r\n\0" as *const u8 as *const libc::c_char,
        (*pool).redis_auth.len,
        (*pool).redis_auth.data,
    );
    if status != 0 as libc::c_int {
        msg_put(msg);
        return status;
    }
    (*msg).set_swallow(1 as libc::c_int as libc::c_uint);
    ((*s_conn).enqueue_inq).unwrap()(ctx, s_conn, msg);
    (*s_conn).set_authenticated(1 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn redis_post_connect(
    mut ctx: *mut context,
    mut conn: *mut conn,
    mut server: *mut server,
) {
    let mut status: rstatus_t = 0;
    let mut pool: *mut server_pool = (*server).owner;
    let mut msg: *mut msg = 0 as *mut msg;
    let mut digits: libc::c_int = 0;
    if (*pool).redis_db <= 0 as libc::c_int {
        return;
    }
    msg = msg_get(conn, 1 as libc::c_int != 0, (*conn).redis() != 0);
    if msg.is_null() {
        return;
    }
    digits = if (*pool).redis_db >= 10 as libc::c_int {
        log10((*pool).redis_db as libc::c_double) as libc::c_int + 1 as libc::c_int
    } else {
        1 as libc::c_int
    };
    status = msg_prepend_format(
        msg,
        b"*2\r\n$6\r\nSELECT\r\n$%d\r\n%d\r\n\0" as *const u8 as *const libc::c_char,
        digits,
        (*pool).redis_db,
    );
    if status != 0 as libc::c_int {
        msg_put(msg);
        return;
    }
    (*msg).type_0 = MSG_REQ_REDIS_SELECT;
    (*msg).result = MSG_PARSE_OK;
    (*msg).set_swallow(1 as libc::c_int as libc::c_uint);
    (*msg).owner = 0 as *mut conn;
    req_server_enqueue_imsgq_head(ctx, conn, msg);
    msg_send(ctx, conn);
}
pub unsafe extern "C" fn redis_swallow_msg(
    mut conn: *mut conn,
    mut pmsg: *mut msg,
    mut msg: *mut msg,
) {
    if !pmsg.is_null()
        && (*pmsg).type_0 as libc::c_uint
            == MSG_REQ_REDIS_SELECT as libc::c_int as libc::c_uint && !msg.is_null()
        && redis_error(msg) as libc::c_int != 0
    {
        let mut conn_server: *mut server = 0 as *mut server;
        let mut conn_pool: *mut server_pool = 0 as *mut server_pool;
        let mut rsp_buffer: *mut mbuf = 0 as *mut mbuf;
        let mut message: [uint8_t; 128] = [0; 128];
        let mut copy_len: size_t = 0;
        conn_server = (*conn).owner as *mut server;
        conn_pool = (*conn_server).owner;
        rsp_buffer = if ((*msg).mhdr.stqh_first).is_null() {
            0 as *mut mbuf
        } else {
            ((*msg).mhdr.stqh_last as *mut libc::c_char)
                .offset(
                    -(&mut (*(0 as *mut libc::c_void as *mut mbuf)).next
                        as *mut C2RustUnnamed_0 as size_t as isize),
                ) as *mut libc::c_void as *mut mbuf
        };
        copy_len = if ((mbuf_length(rsp_buffer))
            .wrapping_sub(3 as libc::c_int as libc::c_uint) as libc::c_ulong)
            < (::std::mem::size_of::<[uint8_t; 128]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            (mbuf_length(rsp_buffer)).wrapping_sub(3 as libc::c_int as libc::c_uint)
                as libc::c_ulong
        } else {
            (::std::mem::size_of::<[uint8_t; 128]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        };
        memcpy(
            message.as_mut_ptr() as *mut libc::c_void,
            &mut *((*rsp_buffer).start).offset(1 as libc::c_int as isize) as *mut uint8_t
                as *const libc::c_void,
            copy_len,
        );
        message[copy_len as usize] = 0 as libc::c_int as uint8_t;
        if log_loggable(4 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_redis.c\0" as *const u8 as *const libc::c_char,
                3241 as libc::c_int,
                0 as libc::c_int,
                b"SELECT %d failed on %s | %s: %s\0" as *const u8 as *const libc::c_char,
                (*conn_pool).redis_db,
                (*conn_pool).name.data,
                (*conn_server).name.data,
                message.as_mut_ptr(),
            );
        }
    }
}
unsafe extern "C" fn run_static_initializers() {
    rsp_no_password = {
        let mut init = string {
            len: (::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
            data: b"-ERR Client sent AUTH, but no password is set\r\n\0" as *const u8
                as *const libc::c_char as *mut uint8_t,
        };
        init
    };
    rsp_invalid_password = {
        let mut init = string {
            len: (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
            data: b"-ERR invalid password\r\n\0" as *const u8 as *const libc::c_char
                as *mut uint8_t,
        };
        init
    };
    rsp_auth_required = {
        let mut init = string {
            len: (::std::mem::size_of::<[libc::c_char; 34]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
            data: b"-NOAUTH Authentication required\r\n\0" as *const u8
                as *const libc::c_char as *mut uint8_t,
        };
        init
    };
    rsp_pong = {
        let mut init = string {
            len: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
            data: b"+PONG\r\n\0" as *const u8 as *const libc::c_char as *mut uint8_t,
        };
        init
    };
    rsp_ok = {
        let mut init = string {
            len: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
            data: b"+OK\r\n\0" as *const u8 as *const libc::c_char as *mut uint8_t,
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
